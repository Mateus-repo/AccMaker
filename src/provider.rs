//! Interface comum a todos os fornecedores de email.
//!
//! A app fala com um `Fornecedor` e não com uma API concreta. Trocar de
//! fornecedor é acrescentar uma implementação desta trait, sem tocar na
//! interface nem nos testes.
//!
//! A trait é deliberadamente pequena: listar, criar, apagar, e dizer quanto
//! cabe e se a password é escolhida pelo utilizador.

use std::fmt;

/// Uma conta de email criada por um fornecedor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inbox {
    pub id: String,
    pub email: String,
    pub criada_em: String,
}

impl Inbox {
    pub fn nova(id: &str, email: &str) -> Self {
        Inbox {
            id: id.to_string(),
            email: email.to_string(),
            criada_em: String::from("2026-01-01T00:00:00Z"),
        }
    }
}

/// O par que a app mostra ao utilizador no fim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credenciais {
    pub email: String,
    /// Password verdadeira quando o fornecedor deixa escolher, a API key quando
    /// nao deixa. Ver `Fornecedor::suporta_password_escolhida`.
    pub password: String,
}

impl Credenciais {
    /// Texto a mostrar ao utilizador para o campo da password. Nos
    /// fornecedores onde a password e uma API key, diz-lo.
    pub fn rotulo_password(&self, password_escolhida: bool) -> &'static str {
        if password_escolhida {
            "Password"
        } else {
            "Chave de acesso (e a password do IMAP)"
        }
    }
}

/// Quantas contas cabem e quantas estao ocupadas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capacidade {
    pub usadas: usize,
    pub maximo: usize,
}

impl Capacidade {
    pub fn livre(&self) -> usize {
        self.maximo.saturating_sub(self.usadas)
    }
    pub fn cheia(&self) -> bool {
        self.usadas >= self.maximo
    }
}

/// Erros do fornecedor, ja com texto pronto para mostrar ao utilizador.
/// Nunca inclui chaves nem tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Erro {
    /// A API key principal esta errada ou sem permissoes.
    CredenciaisInvalidas,
    /// A inbox pedida nao existe (ja foi apagada).
    NaoEncontrado,
    /// Chegou ao limite do plano. A resposta e apagar uma conta, nunca contornar.
    LimiteAtingido { maximo: usize },
    /// Falha de rede. Nao se assume que a operacao teve sucesso.
    Rede,
    /// Demasiados pedidos. O valor vem do cabecalho `Retry-After`, em segundos.
    ExcessoDePedidos { esperar: u64 },
    /// O fornecedor devolveu algo que nao percebo.
    RespostaInesperada,
}

impl fmt::Display for Erro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Erro::CredenciaisInvalidas => {
                write!(f, "A API key esta errada ou nao tem permissoes.")
            }
            Erro::NaoEncontrado => write!(f, "Essa conta ja nao existe."),
            Erro::LimiteAtingido { maximo } => write!(
                f,
                "Chegaste ao limite de {} contas. Apaga uma para criar outra.",
                maximo
            ),
            Erro::Rede => write!(
                f,
                "Sem ligacao ao fornecedor. Nao foi possivel confirmar a operacao."
            ),
            Erro::ExcessoDePedidos { esperar } => {
                write!(f, "Demasiados pedidos. Tenta de novo daqui a {} segundos.", esperar)
            }
            Erro::RespostaInesperada => {
                write!(f, "O fornecedor devolveu uma resposta inesperada.")
            }
        }
    }
}

impl std::error::Error for Erro {}

/// O que a app precisa de qualquer fornecedor.
pub trait Fornecedor {
    /// Nome para mostrar ao utilizador.
    fn nome(&self) -> &'static str;

    /// As contas que existem agora. A API e a fonte da verdade, nunca um
    /// ficheiro local.
    fn listar(&self) -> Result<Vec<Inbox>, Erro>;

    /// Cria uma conta. `username` e opcional: vazio, o fornecedor gera um.
    /// O `client_id` permite repetir o pedido sem criar duplicados.
    fn criar(&self, username: Option<&str>, client_id: &str) -> Result<Credenciais, Erro>;

    /// Apaga uma conta e tudo o que nela ha. Nao ha como anular.
    fn apagar(&self, inbox_id: &str) -> Result<(), Erro>;

    /// Quantas contas cabem. O maximo vem do plano (3 no AgentMail Free).
    fn maximo(&self) -> usize;

    /// `true` se o utilizador escolhe a password (Purelymail), `false` se a
    /// "password" e uma API key gerada pelo fornecedor (AgentMail).
    /// Quando `false`, a interface esconde o gerador e o slider.
    fn suporta_password_escolhida(&self) -> bool {
        true
    }

    /// Quantas contas estao ocupadas e quantas cabem.
    fn capacidade(&self) -> Result<Capacidade, Erro> {
        let usadas = self.listar()?.len();
        Ok(Capacidade {
            usadas,
            maximo: self.maximo(),
        })
    }
}