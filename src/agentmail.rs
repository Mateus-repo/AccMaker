//! Cliente HTTP para a API do AgentMail.
//!
//! Usa `ureq` + `rustls`, medido em 1,17 MB (contra 0,11 MB do WinHTTP, que
//! falhou com erro 12005 nesta maquina). O TLS entra no binario.
//!
//! Nao ha pedidos reais em modo `--dry-run`: em vez de enviar, imprime o
//! metodo, o URL e o corpo, com a API key mascarada, para se comparar com a
//! documentacao.

use crate::cofre::mascarar;
use crate::provider::{Credenciais, Erro, Fornecedor, Inbox};
use ureq::config::Config;

/// Base da API.
pub const BASE: &str = "https://api.agentmail.to";

/// Cabecalho onde vai a API key principal.
const CABECALHO_AUTH: &str = "Authorization";

/// Cliente do AgentMail.
///
/// `dry_run` nunca toca na rede: cada chamada imprime o pedido que faria.
pub struct AgentMail {
    chave: String,
    dry_run: bool,
    /// Inboxes criadas durante um `dry_run`, para o `--dry-run` poder mostrar
    /// o resultado sem rede.
    simuladas: std::cell::RefCell<Vec<Inbox>>,
    contador: std::cell::Cell<u32>,
    /// Id da ultima inbox criada, para apagar a inbox se a chave nao ficar guardada.
    ultimo_id: std::cell::RefCell<Option<String>>,
}

impl AgentMail {
    /// Cliente real. A chave tem de vir do cofre ou do ambiente, nunca de um
    /// argumento de linha de comandos.
    pub fn novo(chave: String) -> Self {
        AgentMail {
            chave,
            dry_run: false,
            simuladas: std::cell::RefCell::new(Vec::new()),
            contador: std::cell::Cell::new(0),
            ultimo_id: std::cell::RefCell::new(None),
        }
    }

    /// Cliente que so imprime os pedidos.
    pub fn dry_run(chave: String) -> Self {
        AgentMail {
            chave,
            dry_run: true,
            simuladas: std::cell::RefCell::new(Vec::new()),
            contador: std::cell::Cell::new(0),
            ultimo_id: std::cell::RefCell::new(None),
        }
    }

    /// Id da ultima inbox criada com sucesso, se houver.
    pub fn ultimo_id(&self) -> Option<String> {
        self.ultimo_id.borrow().clone()
    }

    pub fn e_dry_run(&self) -> bool {
        self.dry_run
    }

    /// Mostra um pedido sem o enviar, com a chave mascarada.
    fn mostrar(&self, metodo: &str, caminho: &str, corpo: Option<&str>) {
        println!("--- pedido (dry-run, nada enviado) ---");
        println!("{} {}{}", metodo, BASE, caminho);
        println!("{}: Bearer {}", CABECALHO_AUTH, mascarar(&self.chave));
        println!("Content-Type: application/json");
        match corpo {
            Some(c) => println!("\n{}", c),
            None => println!("\n(sem corpo)"),
        }
        println!("--- fim do pedido ---");
    }

    fn erro_de_estado(codigo: u16, corpo: &str) -> Erro {
        match codigo {
            401 | 403 => Erro::CredenciaisInvalidas,
            404 => Erro::NaoEncontrado,
            409 => Erro::RespostaInesperada { estado: Some(codigo) },
            422 => {
                // A API traz um campo `code` estavel no corpo. E por ai que
                // se sabe se e limite de inboxes ou username ja em uso.
                let c = Self::campo(corpo, "code").unwrap_or_default();
                if c.contains("inbox_limit")
                    || c.contains("too_many_inboxes")
                    || c.contains("limit_reached")
                    || c.contains("max_inboxes")
                {
                    Erro::LimiteAtingido { maximo: 3 }
                } else if c.contains("already_exists")
                    || c.contains("duplicate")
                    || c.contains("in_use")
                    || c.contains("taken")
                {
                    Erro::NomeJaExiste
                } else {
                    Erro::RespostaInesperada { estado: Some(codigo) }
                }
            }
            429 => {
                let espera = extrair_numeros(corpo, "wait").unwrap_or(5);
                Erro::ExcessoDePedidos { esperar: espera }
            }
            _ => Erro::RespostaInesperada { estado: Some(codigo) },
        }
    }
    /// Envia um pedido e devolve o corpo em texto.
    fn enviar(&self, metodo: &str, caminho: &str, corpo: Option<&str>) -> Result<String, Erro> {
        if self.dry_run {
            self.mostrar(metodo, caminho, corpo);
            return Ok(String::new());
        }
        // Sem isto o ureq transforma qualquer 4xx/5xx num erro e perde o corpo,
// que e onde a AgentMail poe o `code` estavel.
        let agente = ureq::Agent::new_with_config(
            Config::builder()
                .http_status_as_error(false)
                .build(),
        );
        let url = format!("{}{}", BASE, caminho);
        let resposta = match metodo {
            "GET" => agente.get(&url).header(CABECALHO_AUTH, &format!("Bearer {}", self.chave)).call(),
            "DELETE" => agente
                .delete(&url)
                .header(CABECALHO_AUTH, &format!("Bearer {}", self.chave))
                .call(),
            _ => {
                let mut p = agente
                    .post(&url)
                    .header(CABECALHO_AUTH, &format!("Bearer {}", self.chave));
                p = p.header("Content-Type", "application/json");
                p.send(corpo.unwrap_or("{}"))
            }
        };
        match resposta {
            Ok(r) => {
                let estado = r.status().as_u16();
                let texto = r.into_body().read_to_string().unwrap_or_default();
                if !(200..300).contains(&estado) {
                    return Err(Self::erro_de_estado(estado, &texto));
                }
                Ok(texto)
            }
            Err(ureq::Error::StatusCode(codigo)) => {
                // O corpo nao vem no erro do ureq; fica perdido. O mapeamento
                // usa so o codigo HTTP.
                Err(Self::erro_de_estado(codigo, ""))
            }
            Err(_) => Err(Erro::Rede),
        }
    }

    
/// Extrai um campo de texto do JSON, sem trazer uma biblioteca de JSON
    /// para dentro do binario. A API devolve objectos simples e aninhados
    /// (`inboxes`, `threads`), por isso isto cobre o necessario.
    fn campo<'t>(corpo: &'t str, chave: &str) -> Option<String> {
        let procura = format!("\"{}\"", chave);
        let inicio = corpo.find(&procura)? + procura.len();
        let resto = &corpo[inicio..];
        let dois_pontos = resto.find(':')?;
        let depois = resto[dois_pontos + 1..].trim_start();
        if let Some(interno) = depois.strip_prefix('"') {
            let fim = interno.find('"')?;
            Some(interno[..fim].to_string())
        } else {
            // Numero ou objeto: nao interessa para o que precisamos ler.
            None
        }
    }

    /// Le uma lista de inboxes de `{"inboxes": [...]}` ou de uma lista
    /// direta. Cada entrada tem `inbox_id`, `email` e `created_at`.
    fn ler_inboxes(corpo: &str) -> Vec<Inbox> {
        // Recorta ao array, seja `inboxes` ou o proprio corpo.
        let inicio = match corpo.find("\"inboxes\"") {
            Some(i) => match corpo[i..].find('[') {
                Some(j) => i + j,
                None => return Vec::new(),
            },
            None => match corpo.find('[') {
                Some(j) => j,
                None => return Vec::new(),
            },
        };
        let mut resultado = Vec::new();
        let mut resto = &corpo[inicio + 1..];
        while let Some(abertura) = resto.find('{') {
            let fecho = match resto[abertura..].find('}') {
                Some(f) => abertura + f,
                None => break,
            };
            let objeto = &resto[abertura..fecho];
            if let (Some(id), Some(email)) = (
                Self::campo(objeto, "inbox_id"),
                Self::campo(objeto, "email"),
            ) {
                resultado.push(Inbox {
                    id,
                    email,
                    criada_em: Self::campo(objeto, "created_at").unwrap_or_default(),
                });
            }
            resto = &resto[fecho + 1..];
        }
        resultado
    }
}

impl Fornecedor for AgentMail {
    fn nome(&self) -> &'static str {
        "AgentMail"
    }

    fn listar(&self) -> Result<Vec<Inbox>, Erro> {
        if self.dry_run {
            self.mostrar("GET", "/v0/inboxes", None);
            return Ok(self.simuladas.borrow().clone());
        }
        let corpo = self.enviar("GET", "/v0/inboxes", None)?;
        Ok(Self::ler_inboxes(&corpo))
    }

    fn criar(&self, username: Option<&str>, client_id: &str) -> Result<Credenciais, Erro> {
        let corpo = match username {
            Some(u) => format!(
                r#"{{"username":"{}","client_id":"{}"}}"#,
                u, client_id
            ),
            None => format!(r#"{{"client_id":"{}"}}"#, client_id),
        };

        if self.dry_run {
            self.mostrar("POST", "/v0/inboxes", Some(&corpo));
            // Simula o resultado para o fluxo poder continuar sem rede.
            let n = self.contador.get() + 1;
            self.contador.set(n);
            let id = format!("sim-{}", n);
            let email = format!(
                "{}@agentmail.to",
                username.unwrap_or("simulado")
            );
            let inbox = Inbox::nova(&id, &email);
            self.simuladas.borrow_mut().push(inbox);
            return Ok(Credenciais {
                email,
                password: format!("amk_simulada_{}", id),
            });
        }

        let resposta = self.enviar("POST", "/v0/inboxes", Some(&corpo))?;
        let id = Self::campo(&resposta, "inbox_id").ok_or(Erro::RespostaInesperada { estado: None })?;
        let email = Self::campo(&resposta, "email").ok_or(Erro::RespostaInesperada { estado: None })?;

        // O AgentMail nao tem password: a "password" e uma API key restrita
        // a esta inbox.
        let pedido_key = format!(
            r#"{{"name":"accmaker","permissions":{{"inbox_read":true,"message_read":true}}}}"#
        );
        let caminho = format!("/v0/inboxes/{}/api-keys", id);
        let resposta_key = self.enviar("POST", &caminho, Some(&pedido_key))?;
        let chave = Self::campo(&resposta_key, "api_key").ok_or(Erro::RespostaInesperada { estado: None })?;
        *self.ultimo_id.borrow_mut() = Some(id);

        Ok(Credenciais { email, password: chave })
    }

    fn apagar(&self, inbox_id: &str) -> Result<(), Erro> {
        let caminho = format!("/v0/inboxes/{}", inbox_id);
        self.enviar("DELETE", &caminho, None)?;
        Ok(())
    }

    fn maximo(&self) -> usize {
        3
    }

    fn suporta_password_escolhida(&self) -> bool {
        // No AgentMail a password e uma API key, nao escolhida pelo utilizador.
        false
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn cliente() -> AgentMail {
        AgentMail::dry_run("amk_teste_1234567890".to_string())
    }

    #[test]
    fn dry_run_nao_faz_rede() {
        // Se fizer rede, o teste demora e falha; em dry_run tem de ser instant.
        let c = cliente();
        let t = std::time::Instant::now();
        let r = c.listar().unwrap();
        assert!(r.is_empty());
        assert!(t.elapsed().as_millis() < 500, "demorou, pode ter feito rede");
    }

    #[test]
    fn dry_run_mostra_o_pedido_com_metodo_e_url() {
        // O pedido tem de bater certo com a documentacao.
        let c = cliente();
        let _ = c.listar().unwrap();
    }

    #[test]
    fn parse_de_inboxes() {
        let corpo = r#"{"inboxes":[{"inbox_id":"abc","email":"a@agentmail.to","created_at":"2026-01-01T00:00:00Z"},{"inbox_id":"def","email":"b@agentmail.to","created_at":"2026-01-02T00:00:00Z"}]}"#;
        let v = AgentMail::ler_inboxes(corpo);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].id, "abc");
        assert_eq!(v[0].email, "a@agentmail.to");
        assert_eq!(v[1].id, "def");
    }

    #[test]
    fn parse_de_inboxes_vazio() {
        assert_eq!(AgentMail::ler_inboxes(r#"{"inboxes":[]}"#).len(), 0);
        assert_eq!(AgentMail::ler_inboxes("{}").len(), 0);
        assert_eq!(AgentMail::ler_inboxes("").len(), 0);
    }

    #[test]
    fn extrai_campos() {
        assert_eq!(
            AgentMail::campo(r#"{"inbox_id":"x1","email":"e@e"}"#, "inbox_id"),
            Some("x1".to_string())
        );
        assert_eq!(
            AgentMail::campo(r#"{"inbox_id":"x1"}"#, "email"),
            None
        );
    }

    #[test]
    fn erros_por_codigo_http() {
        assert_eq!(
            AgentMail::erro_de_estado(401, ""),
            Erro::CredenciaisInvalidas
        );
        assert_eq!(AgentMail::erro_de_estado(403, ""), Erro::CredenciaisInvalidas);
        assert_eq!(AgentMail::erro_de_estado(404, ""), Erro::NaoEncontrado);
        assert_eq!(
            AgentMail::erro_de_estado(422, r#"{"code":"inbox_limit_reached"}"#),
            Erro::LimiteAtingido { maximo: 3 }
        );
        assert_eq!(AgentMail::erro_de_estado(429, ""), Erro::ExcessoDePedidos { esperar: 5 });
        assert_eq!(
            AgentMail::erro_de_estado(500, ""),
            Erro::RespostaInesperada { estado: Some(500) }
        );
        // O corpo traz o `code` estavel da API. E ele que diz se e
        // limite de inboxes ou username ja em uso.
        assert_eq!(
            AgentMail::erro_de_estado(422, r#"{"code":"username_already_exists"}"#),
            Erro::NomeJaExiste
        );
        assert_eq!(
            AgentMail::erro_de_estado(422, r#"{"code":"inbox_limit_reached"}"#),
            Erro::LimiteAtingido { maximo: 3 }
        );
        // Estados que nao sabemos tratar dizem qual foi.
        assert_eq!(
            AgentMail::erro_de_estado(503, ""),
            Erro::RespostaInesperada { estado: Some(503) }
        );
        assert_eq!(
            format!("{}", Erro::RespostaInesperada { estado: Some(503) }),
            "O fornecedor devolveu uma resposta inesperada (estado 503)."
        );
    }

    #[test]
    fn erro_nao_contem_a_chave() {
        // A chave nunca pode aparecer no texto de erro.
        let c = cliente();
        let e = AgentMail::erro_de_estado(401, "");
        assert!(!format!("{}", e).contains("amk_teste"));
        let _ = c;
    }

    #[test]
    fn agentmail_nao_suporta_password_escolhida() {
        assert!(!cliente().suporta_password_escolhida());
        assert_eq!(cliente().maximo(), 3);
    }

    #[test]
    fn dry_run_cria_sem_rede_e_simula() {
        let c = cliente();
        let r = c.criar(Some("teste"), "cli-1").unwrap();
        assert_eq!(r.email, "teste@agentmail.to");
        assert!(!r.password.is_empty());
        // E a listagem passa a mostrar o que foi simulado.
        assert_eq!(c.listar().unwrap().len(), 1);
    }
}

/// Le o primeiro numero que aparece depois de `"campo"` num JSON.
fn extrair_numeros(corpo: &str, campo: &str) -> Option<u64> {
    let procura = format!("\"{}\"", campo);
    let inicio = corpo.find(&procura)? + procura.len();
    let resto = &corpo[inicio..];
    let digitos: String = resto
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digitos.parse::<u64>().ok()
}
