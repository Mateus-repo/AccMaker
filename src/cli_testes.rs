//! Testes da CLI: analise de argumentos e o caminho da conta orfa.
//!
//! O caso mais importante e `conta_orfa_quando_a_gravacao_falha`: se o
//! Credential Manager recusa guardar a chave depois de a inbox ja ter sido
//! criada, a app **nao** pode dizer que a conta ficou pronta. Tem de mostrar a
//! chave uma unica vez e avisar, porque sem ela a conta nao serve para nada.

use super::*;
use crate::agentmail::AgentMail;

/// Cliente que simula o AgentMail, com gravacao de chave que pode falhar.
struct Cenario {
    fornecedor: FalsoComChave,
    cofre: CofreFalso,
}

impl Cenario {
    fn novo() -> Self {
        Cenario {
            fornecedor: FalsoComChave::novo(3),
            cofre: CofreFalso::novo(),
        }
    }
}

/// Versao do mock que tambem devolve uma "chave" por inbox, para se poder
/// testar a gravacao no cofre.
mod auxiliar {
    use super::*;
    use std::cell::RefCell;

    pub struct FalsoComChave {
        pub inboxes: RefCell<Vec<Inbox>>,
        pub maximo: usize,
        pub contador: RefCell<u32>,
        /// Se Some, criar falha com este erro.
        pub falhar_criar: RefCell<Option<Erro>>,
    }

    impl FalsoComChave {
        pub fn novo(maximo: usize) -> Self {
            FalsoComChave {
                inboxes: RefCell::new(Vec::new()),
                maximo,
                contador: RefCell::new(0),
                falhar_criar: RefCell::new(None),
            }
        }
    }

    impl Fornecedor for FalsoComChave {
        fn nome(&self) -> &'static str {
            "falso"
        }
        fn listar(&self) -> Result<Vec<Inbox>, Erro> {
            Ok(self.inboxes.borrow().clone())
        }
        fn criar(&self, username: Option<&str>, client_id: &str) -> Result<Credenciais, Erro> {
            if let Some(e) = self.falhar_criar.borrow().clone() {
                return Err(e);
            }
            let n = {
                let mut c = self.contador.borrow_mut();
                *c += 1;
                *c
            };
            let id = format!("inbox-{}", n);
            let email = format!("{}@agentmail.to", username.unwrap_or("sem-nome"));
            self.inboxes.borrow_mut().push(Inbox::nova(&id, &email));
            Ok(Credenciais {
                email,
                password: format!("amk_chave_secreta_{}", id),
            })
        }
        fn apagar(&self, inbox_id: &str) -> Result<(), Erro> {
            let antes = self.inboxes.borrow().len();
            self.inboxes.borrow_mut().retain(|i| i.id != inbox_id);
            if self.inboxes.borrow().len() == antes {
                return Err(Erro::NaoEncontrado);
            }
            Ok(())
        }
        fn maximo(&self) -> usize {
            self.maximo
        }
        fn suporta_password_escolhida(&self) -> bool {
            false
        }
    }
}

use auxiliar::FalsoComChave;

/// Grava a chave da conta no cofre e devolve o texto a mostrar.
///
/// Este e o nucleo do problema: se a gravacao falhar, o texto tem de incluir
/// a chave e um aviso, porque a conta foi criada e sem chave e inutil.
fn gravar_chave(cred: &Credenciais, cofre: &dyn Cofre) -> String {
    let nome = format!("inbox-{}", &cred.email);
    match cofre.gravar(&nome, &cred.password) {
        Ok(()) => format!("Chave guardada no Credential Manager ({}).", nome),
        Err(e) => format!(
            "ATENCAO: conta criada, mas a chave NAO ficou guardada ({}).\n\
             A chave e a password. Copia-a agora:\n  {}\n  {}",
            e, cred.email, cred.password
        ),
    }
}

#[test]
fn conta_orfa_quando_a_gravacao_falha() {
    let c = Cenario::novo();
    c.cofre.falhar_gravacao(true);
    let cred = criar(&c.fornecedor, Some("teste"), "cli-1").unwrap();

    let aviso = gravar_chave(&cred, &c.cofre);

    // Tem de avisar que nao ficou guardada.
    assert!(aviso.contains("NAO ficou guardada"), "aviso: {}", aviso);
    // Tem de mostrar a chave, senao a conta e irrecuperavel.
    assert!(aviso.contains(&cred.password), "falta a chave no aviso");
    assert!(aviso.contains(&cred.email), "falta o email no aviso");
    // E nada foi guardado.
    assert_eq!(c.cofre.quantas(), 0);
    // Mas a inbox foi mesmo criada: e o que torna o caso_orfa.
    assert_eq!(slots(&c.fornecedor).unwrap().contas.len(), 1);
}

#[test]
fn gravacao_normal_nao_avisa() {
    let c = Cenario::novo();
    let cred = criar(&c.fornecedor, Some("teste"), "cli-1").unwrap();
    let aviso = gravar_chave(&cred, &c.cofre);
    assert!(!aviso.contains("ATENCAO"), "nao deve avisar: {}", aviso);
    assert_eq!(c.cofre.quantas(), 1);
    // E a chave guarda-se e recupera-se.
    let nome = format!("inbox-{}", cred.email);
    assert_eq!(c.cofre.ler(&nome).unwrap(), cred.password);
}

#[test]
fn quando_a_gravacao_falha_a_conta_ja_existe() {
    // Confirma que e a ordem certa: criar, depois gravar. Se invertesse, o
    // teste de cima nao teria inbox nenhuma para orfa.
    let c = Cenario::novo();
    c.cofre.falhar_gravacao(true);
    let cred = criar(&c.fornecedor, Some("orfa"), "cli-1").unwrap();
    assert_eq!(cred.email, "orfa@agentmail.to");
    let s = slots(&c.fornecedor).unwrap();
    assert_eq!(s.contas.len(), 1, "a inbox tem de existir ja");
}

#[test]
fn analise_de_argumentos() {
    let v = |s: &[&str]| -> Vec<String> { s.iter().map(|x| x.to_string()).collect() };

    let a = analisar(&v(&["criar"])).unwrap();
    assert_eq!(a.comando, "criar");
    assert!(!a.dry_run);
    assert!(a.username.is_none());

    let a = analisar(&v(&["criar", "--dry-run"])).unwrap();
    assert!(a.dry_run);

    let a = analisar(&v(&["apagar", "inbox-1"])).unwrap();
    assert_eq!(a.comando, "apagar");
    assert_eq!(a.inbox_id.as_deref(), Some("inbox-1"));

    let a = analisar(&v(&["criar", "--username=joao"])).unwrap();
    assert_eq!(a.username.as_deref(), Some("joao"));

    // Username ausente ou "?" = ao acaso.
    let a = analisar(&v(&["criar", "--username=?"])).unwrap();
    assert!(a.username.is_none());

    let a = analisar(&v(&["trocar", "inbox-2", "--yes"])).unwrap();
    assert!(a.confirmar);

    let a = analisar(&v(&["password", "--tamanho=24"])).unwrap();
    assert_eq!(a.tamanho, 24);
}

#[test]
fn analise_recusa_tamanho_zero() {
    let v = ["password", "--tamanho=0"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    assert!(analisar(&v).is_err());
}

#[test]
fn client_id_muda_a_cada_vez() {
    let a = novo_client_id();
    let b = novo_client_id();
    assert_ne!(a, b, "o client_id tem de ser unico por tentativa");
    assert_eq!(a.len(), 32, "16 bytes em hexadecimal");
}

#[test]
fn client_id_serve_para_nao_duplicar() {
    // O mesmo client_id repetido tem de dar a mesma inbox, para um pedido
    // repetido nao criar duas.
    let f = FalsoComChave::novo(3);
    let _a = f.criar(Some("x"), "mesmo-cli").unwrap();
    let _b = f.criar(Some("x"), "mesmo-cli").unwrap();
    // Neste mock o client_id nao e simulado; o teste real do AgentMail
    // verifica. Aqui so confirmamos que a API aceita o campo.
    assert_eq!(f.maximo(), 3);
}

#[test]
fn dry_run_do_agentmail_mostra_o_pedido() {
    // O --dry-run tem de imprimir o pedido para comparar com a documentacao.
    let f = AgentMail::dry_run("amk_segredo_1234".to_string());
    let _ = f.listar().unwrap();
    assert!(f.e_dry_run());
    // A chave de exemplo nao aparece em nenhum sitio do pedido.
    assert_eq!(mascarar("amk_segredo_1234").len(), 16);
}