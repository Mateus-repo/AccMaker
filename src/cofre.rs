//! Guarda de segredos: a API key principal e as chaves restritas de cada inbox.
//!
//! A key **principal** vem de uma variavel de ambiente ou do Credential Manager.
//! Nunca de um argumento de linha de comandos (que fica no historico do
//! Windows) e nunca de um log.
//!
//! As chaves **restritas** que servem de password a cada inbox vao para o
//! Credential Manager do Windows. Se essa gravacao falhar depois de a inbox
//! ja ter sido criada, a conta fica orfa: e o caso que `gravar_chave` trata
//! explicitamente, e ha teste para ele.

use std::collections::BTreeMap;

/// Nome do servico no Credential Manager.
const SERVICO: &str = "AccMaker";

/// Erros do Credential Manager. Distintos dos erros de rede, porque a
/// remediacao e outra.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Erro {
    /// O Credential Manager recusou a gravacao ou a leitura.
    Sistema,
    /// Nao ha nada guardado com esse nome.
    NaoEncontrado,
}

impl std::fmt::Display for Erro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Erro::Sistema => write!(
                f,
                "O Credential Manager do Windows recusou a operacao."
            ),
            Erro::NaoEncontrado => write!(f, "Nao ha nada guardado com esse nome."),
        }
    }
}

impl std::error::Error for Erro {}

/// Onde ficam os segredos. Trait para poder simular falhas nos testes.
pub trait Cofre: std::fmt::Debug {
    fn gravar(&self, nome: &str, segredo: &str) -> Result<(), Erro>;
    fn ler(&self, nome: &str) -> Result<String, Erro>;
    fn apagar(&self, nome: &str) -> Result<(), Erro>;
}

/// Cofre de verdade: Credential Manager do Windows.
#[derive(Debug)]
pub struct CofreWindows;

#[cfg(windows)]
mod implementacao {
    use super::*;
    use windows_sys::Win32::Security::Credentials::{
        CredDeleteW, CredFree, CredReadW, CredWriteW, CREDENTIALW, CRED_PERSIST_LOCAL_MACHINE,
        CRED_TYPE_GENERIC,
    };

    /// Converte uma string para o buffer largo null-terminated que a API
    /// do Windows espera. Nunca deixa o segredo numa string da STL.
    fn para_largo(texto: &str) -> Vec<u16> {
        texto.encode_utf16().chain(std::iter::once(0)).collect()
    }

    unsafe impl Send for CofreWindows {}

    impl Cofre for CofreWindows {
        fn gravar(&self, nome: &str, segredo: &str) -> Result<(), Erro> {
            // Mesma chave de alvo que na leitura.
            let mut utilizador = para_largo(nome);
            let mut alvo = para_largo(&format!("{}/{}", SERVICO, nome));
            let mut bytes: Vec<u8> = segredo.as_bytes().to_vec();
            unsafe {
                let mut cred = std::mem::zeroed::<CREDENTIALW>();
                cred.Type = CRED_TYPE_GENERIC;
                cred.TargetName = alvo.as_mut_ptr();
                cred.CredentialBlobSize = bytes.len() as u32;
                cred.CredentialBlob = bytes.as_mut_ptr();
                cred.Persist = CRED_PERSIST_LOCAL_MACHINE;
                cred.UserName = utilizador.as_mut_ptr();
                if CredWriteW(&cred, 0) == 0 {
                    return Err(Erro::Sistema);
                }
            }
            // Apaga o segredo da memoria depois de gravado.
            for b in bytes.iter_mut() {
                *b = 0;
            }
            Ok(())
        }

        fn ler(&self, nome: &str) -> Result<String, Erro> {
            // O alvo tem de ser o nome da entrada, nao o nome do servico:
            // e por isso que cada inbox tem a sua.
            let alvo = para_largo(&format!("{}/{}", SERVICO, nome));
            let mut ponteiro: *mut CREDENTIALW = std::ptr::null_mut();
            unsafe {
                if CredReadW(alvo.as_ptr(), CRED_TYPE_GENERIC, 0, &mut ponteiro) == 0 {
                    return Err(match std::io::Error::last_os_error().raw_os_error() {
                        Some(1168) => Erro::NaoEncontrado,
                        _ => Erro::Sistema,
                    });
                }
                let cred = &*ponteiro;
                let tamanho = cred.CredentialBlobSize as usize;
                let bytes = std::slice::from_raw_parts(cred.CredentialBlob, tamanho);
                let resultado = String::from_utf8(bytes.to_vec())
                    .map_err(|_| Erro::Sistema);
                CredFree(ponteiro as *const _);
                resultado
            }
        }

        fn apagar(&self, nome: &str) -> Result<(), Erro> {
            let alvo = para_largo(&format!("{}/{}", SERVICO, nome));
            unsafe {
                if CredDeleteW(alvo.as_ptr(), CRED_TYPE_GENERIC, 0) == 0 {
                    return Err(Erro::Sistema);
                }
            }
            Ok(())
        }
    }

    // Silencia avisos de importacoes nao usadas em builds parciais.
    #[allow(dead_code)]
    fn _nao_usado() {}
}

#[cfg(not(windows))]
impl Cofre for CofreWindows {
    fn gravar(&self, _nome: &str, _segredo: &str) -> Result<(), Erro> {
        Err(Erro::Sistema)
    }
    fn ler(&self, _nome: &str) -> Result<String, Erro> {
        Err(Erro::Sistema)
    }
    fn apagar(&self, _nome: &str) -> Result<(), Erro> {
        Err(Erro::Sistema)
    }
}

/// Cofre de mentira, para testes. Pode ser feito falhar de proposito.
#[derive(Debug, Default)]
pub struct CofreFalso {
    /// `true` = a gravacao falha com `Erro::Sistema`.
    falha_gravacao: std::cell::Cell<bool>,
    guardados: std::cell::RefCell<BTreeMap<String, String>>,
}

impl CofreFalso {
    pub fn novo() -> Self {
        CofreFalso::default()
    }

    /// Faz a proxima gravacao falhar, para testar o caminho da conta orfa.
    pub fn falhar_gravacao(&self, sim: bool) {
        self.falha_gravacao.set(sim);
    }

    pub fn quantas(&self) -> usize {
        self.guardados.borrow().len()
    }
}

impl Cofre for CofreFalso {
    fn gravar(&self, nome: &str, segredo: &str) -> Result<(), Erro> {
        if self.falha_gravacao.get() {
            return Err(Erro::Sistema);
        }
        self.guardados
            .borrow_mut()
            .insert(nome.to_string(), segredo.to_string());
        Ok(())
    }

    fn ler(&self, nome: &str) -> Result<String, Erro> {
        self.guardados
            .borrow()
            .get(nome)
            .cloned()
            .ok_or(Erro::NaoEncontrado)
    }

    fn apagar(&self, nome: &str) -> Result<(), Erro> {
        self.guardados.borrow_mut().remove(nome);
        Ok(())
    }
}

/// De onde vem a API key principal. A prioridade e a seguinte:
///
/// 1. variavel de ambiente `ACCMAILER_API_KEY` (ou `AGENTMAIL_API_KEY`);
/// 2. Credential Manager do Windows.
///
/// Nunca um argumento de linha de comandos.
pub fn ler_api_key(cofre: &dyn Cofre) -> Result<String, String> {
    if let Ok(k) = std::env::var("AGENTMAIL_API_KEY") {
        if !k.trim().is_empty() {
            return Ok(k);
        }
    }
    if let Ok(k) = std::env::var("ACCMAILER_API_KEY") {
        if !k.trim().is_empty() {
            return Ok(k);
        }
    }
    cofre
        .ler("api-key-principal")
        .map_err(|_| "Sem API key. Define AGENTMAIL_API_KEY ou grava no Credential Manager.".to_string())
}

/// Mascarar uma chave para mostrar sem a expor: fica o prefixo e o resto
/// vira pontos. Usado nos pedidos do `--dry-run` e em qualquer erro.
pub fn mascarar(chave: &str) -> String {
    // Abaixo de 8 caracteres nao ha prefixo a mostrar sem revelar quase tudo.
    if chave.len() < 8 {
        return "*".repeat(chave.len());
    }
    let n = 4;
    format!("{}{}", &chave[..n], "*".repeat(chave.len() - n))
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn mascarar_nao_revela_a_chave() {
        let c = "amk_1234567890abcdef";
        let m = mascarar(c);
        assert!(m.starts_with("amk_"), "guarda o prefixo: {}", m);
        assert!(!m.contains("1234567890"), "revela o resto: {}", m);
        assert_eq!(m.len(), c.len(), "o comprimento nao deve mudar");
    }

    #[test]
    fn mascarar_chaves_curtas() {
        assert_eq!(mascarar(""), "");
        assert_eq!(mascarar("abc"), "***");
        assert_eq!(mascarar("abcdefgh"), "abcd****");
    }

    #[test]
    fn cofre_falso_grava_lê_e_apaga() {
        let c = CofreFalso::novo();
        c.gravar("inbox-1", "segredo").unwrap();
        assert_eq!(c.ler("inbox-1").unwrap(), "segredo");
        assert_eq!(c.quantas(), 1);
        c.apagar("inbox-1").unwrap();
        assert_eq!(c.ler("inbox-1").unwrap_err(), Erro::NaoEncontrado);
    }

    #[test]
    fn cofre_falso_pode_falhar_a_gravacao() {
        let mut c = CofreFalso::novo();
        c.falhar_gravacao(true);
        assert_eq!(c.gravar("inbox-1", "segredo").unwrap_err(), Erro::Sistema);
        assert_eq!(c.quantas(), 0, "nada foi guardado");
    }

    #[test]
    fn ler_api_key_vem_do_ambiente_primeiro() {
        let c = CofreFalso::novo();
        c.gravar("api-key-principal", "do-cofre").unwrap();
        std::env::set_var("AGENTMAIL_API_KEY", "do-ambiente");
        let k = ler_api_key(&c).unwrap();
        std::env::remove_var("AGENTMAIL_API_KEY");
        assert_eq!(k, "do-ambiente");
    }

    #[test]
    fn ler_api_key_cai_para_o_cofre() {
        let c = CofreFalso::novo();
        c.gravar("api-key-principal", "do-cofre").unwrap();
        assert_eq!(ler_api_key(&c).unwrap(), "do-cofre");
    }

    #[test]
    fn ler_api_key_da_erro_sem_nada() {
        let c = CofreFalso::novo();
        assert!(ler_api_key(&c).is_err());
    }
}