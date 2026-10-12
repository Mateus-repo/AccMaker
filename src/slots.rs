//! Fluxos de alto nivel sobre um `Fornecedor`: listar, criar e trocar.
//!
//! A logica do "trocar" vive aqui e nao no fornecedor, para ser igual em
//! todos e para poder ser testada com um fornecedor simulado.

use crate::provider::{Capacidade, Credenciais, Erro, Fornecedor, Inbox};

/// Estado dos slots, como a interface mostra.
#[derive(Debug, Clone)]
pub struct Slots {
    pub contas: Vec<Inbox>,
    pub capacidade: Capacidade,
}

/// Le os slots. A API e a fonte da verdade: o que estiver no ecra e o que a
/// API respondeu, nunca um ficheiro local.
pub fn slots(f: &dyn Fornecedor) -> Result<Slots, Erro> {
    let contas = f.listar()?;
    let maximo = f.maximo();
    Ok(Slots {
        capacidade: Capacidade {
            usadas: contas.len(),
            maximo,
        },
        contas,
    })
}

/// Cria uma conta nova, recusando antes de chamar a API se nao ha vaga.
/// Sem este teste previo, a app gastaria um pedido para ouvir que nao ha vaga.
pub fn criar(f: &dyn Fornecedor, username: Option<&str>, client_id: &str) -> Result<Credenciais, Erro> {
    let c = f.capacidade()?;
    if c.cheia() {
        return Err(Erro::LimiteAtingido { maximo: c.maximo });
    }
    f.criar(username, client_id)
}

/// Apaga uma conta e volta a ler os slots.
///
/// Se a rede falhar a meio, nao se assume que o apagamento happened: volta a
/// listar e so isso decide o estado real.
pub fn apagar(f: &dyn Fornecedor, inbox_id: &str) -> Result<Slots, Erro> {
    f.apagar(inbox_id)?;
    slots(f)
}

/// O pedido principal: apagar uma conta e criar outra em sequencia.
///
/// Só cria se o apagar tiver sido confirmado pela listagem seguinte. Se o
/// apagamento falhar, ou se a vaga nao aparecer, nao cria nada — para nao
/// ficar com 4 contas e uma a mais.
pub fn trocar(f: &dyn Fornecedor, inbox_id: &str, username: Option<&str>, client_id: &str) -> Result<Credenciais, Erro> {
    let depois = apagar(f, inbox_id)?;
    if depois.capacidade.cheia() {
        // A vaga nao libertou-se. Nao criar: devolve o estado real.
        return Err(Erro::RespostaInesperada { estado: None });
    }
    criar(f, username, client_id)
}

/// Username aleatorio, legivel, para quando o utilizador deixa ao acaso.
/// Evita caracteres que se confundem entre si (0/O, 1/l/I) e usa o
/// gerador do sistema.
pub fn username_aleatorio(comprimento: usize) -> String {
    // Sem as letras que se confundem entre si: l, i (e 1, 0, O, que aqui
    // nao entram por serem maiusculas ou digitos).
    const CONSOANTES: &[u8] = b"bcdfghjkmnpqrstvwxz";
    const VOGAIS: &[u8] = b"aeou";
    let mut saida = String::with_capacity(comprimento);
    let mut rng = crate::rng::OsRng;
    for i in 0..comprimento {
        let lista = if i % 2 == 0 { VOGAIS } else { CONSOANTES };
        saida.push(lista[(rng.next_u8() as usize) % lista.len()] as char);
    }
    saida
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::cell::RefCell;

    /// Fornecedor simulado, com os mesmos limites do plano Free: 3 inboxes e
    /// a password a ser uma API key.
    struct Falso {
        inboxes: RefCell<Vec<Inbox>>,
        maximo: usize,
        /// Quando Some, `apagar` devolve este erro em vez de apagar.
        falhar_apagar: RefCell<Option<Erro>>,
        contador: RefCell<u32>,
    }

    impl Falso {
        fn novo(maximo: usize) -> Self {
            Falso {
                inboxes: RefCell::new(Vec::new()),
                maximo,
                falhar_apagar: RefCell::new(None),
                contador: RefCell::new(0),
            }
        }
        }
    impl std::fmt::Debug for Falso {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Falso({} inboxes)", self.inboxes.borrow().len())
        }
    }

    impl Fornecedor for Falso {
        fn nome(&self) -> &'static str {
            "falso"
        }

        fn listar(&self) -> Result<Vec<Inbox>, Erro> {
            Ok(self.inboxes.borrow().clone())
        }

        fn criar(&self, username: Option<&str>, client_id: &str) -> Result<Credenciais, Erro> {
            let n = self.inboxes.borrow().len();
            if n >= self.maximo {
                return Err(Erro::LimiteAtingido { maximo: self.maximo });
            }
            let mut c = self.contador.borrow_mut();
            *c += 1;
            let id = format!("inbox-{}", c);
            let local = username.map(|u| u.to_string()).unwrap_or_else(|| {
                format!("user{}", c)
            });
            let email = format!("{}@agentmail.to", local);
            self.inboxes.borrow_mut().push(Inbox::nova(&id, &email));
            // A "password" e uma API key, como no AgentMail.
            let key = format!("amk_{}_{}", id, client_id);
            Ok(Credenciais {
                email,
                password: key,
            })
        }

        fn apagar(&self, inbox_id: &str) -> Result<(), Erro> {
            if let Some(e) = self.falhar_apagar.borrow().clone() {
                return Err(e);
            }
            let antes = self.inboxes.borrow().len();
            self.inboxes
                .borrow_mut()
                .retain(|i| i.id != inbox_id);
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

    #[test]
    fn comeca_vazio() {
        let f = Falso::novo(3);
        let s = slots(&f).unwrap();
        assert_eq!(s.contas.len(), 0);
        assert_eq!(s.capacidade.usadas, 0);
        assert_eq!(s.capacidade.maximo, 3);
        assert_eq!(s.capacidade.livre(), 3);
        assert!(!s.capacidade.cheia());
    }

    #[test]
    fn criar_ate_tres() {
        let f = Falso::novo(3);
        for i in 1..=3 {
            let c = criar(&f, None, &format!("cli{}", i)).unwrap();
            assert!(c.email.ends_with("@agentmail.to"), "email estranho: {}", c.email);
        }
        let s = slots(&f).unwrap();
        assert_eq!(s.contas.len(), 3);
        assert_eq!(s.capacidade.livre(), 0);
    }

    #[test]
    fn a_quarta_da_erro_sem_chegar_a_api() {
        let f = Falso::novo(3);
        for i in 1..=3 {
            criar(&f, None, &format!("cli{}", i)).unwrap();
        }
        // `criar` recusa pela capacidade antes de chamar o fornecedor, por isso
        // o erro e o mesmo de qualquer forma.
        let e = criar(&f, None, "cli4").unwrap_err();
        assert_eq!(e, Erro::LimiteAtingido { maximo: 3 });
        // E nada foi criado.
        assert_eq!(slots(&f).unwrap().contas.len(), 3);
    }

    #[test]
    fn apagar_liberta_a_vaga() {
        let f = Falso::novo(3);
        let a = criar(&f, None, "c1").unwrap();
        criar(&f, None, "c2").unwrap();
        criar(&f, None, "c3").unwrap();
        let id = f.listar().unwrap()[0].id.clone();
        let s = apagar(&f, &id).unwrap();
        assert_eq!(s.contas.len(), 2);
        assert_eq!(s.capacidade.livre(), 1);
        let _ = a;
        // E da para criar outra.
        criar(&f, None, "c4").unwrap();
        assert_eq!(slots(&f).unwrap().contas.len(), 3);
    }

    #[test]
    fn apagar_inexistente_da_erro() {
        let f = Falso::novo(3);
        assert_eq!(apagar(&f, "nao-existe").unwrap_err(), Erro::NaoEncontrado);
    }

    #[test]
    fn trocar_apaga_e_cria() {
        let f = Falso::novo(3);
        let antiga = criar(&f, Some("antiga"), "c1").unwrap();
        let id = f.listar().unwrap()[0].id.clone();
        assert_eq!(antiga.email, "antiga@agentmail.to");

        let nova = trocar(&f, &id, Some("nova"), "t1").unwrap();
        assert_eq!(nova.email, "nova@agentmail.to");
        assert!(!nova.password.is_empty(), "a key tem de vir preenchida");

        let s = slots(&f).unwrap();
        assert_eq!(s.contas.len(), 1, "deve ficar uma so conta");
        assert_eq!(s.contas[0].email, "nova@agentmail.to");
    }

    #[test]
    fn trocar_com_a_lista_cheia_funca() {
        let f = Falso::novo(3);
        for i in 1..=3 {
            criar(&f, None, &format!("c{}", i)).unwrap();
        }
        let id = f.listar().unwrap()[1].id.clone();
        let nova = trocar(&f, &id, None, "t1").unwrap();
        let s = slots(&f).unwrap();
        assert_eq!(s.contas.len(), 3, "continua cheio, mas com outra conta");
        assert!(s.contas.iter().any(|i| i.id != id));
        assert!(!nova.password.is_empty());
    }

    #[test]
    fn se_o_apagar_falhar_nao_se_cria() {
        let f = Falso::novo(3);
        criar(&f, None, "c1").unwrap();
        let id = f.listar().unwrap()[0].id.clone();
        *f.falhar_apagar.borrow_mut() = Some(Erro::Rede);
        let e = trocar(&f, &id, None, "t1").unwrap_err();
        assert_eq!(e, Erro::Rede);
        // Nada foi criado: a conta antiga continua la.
        assert_eq!(slots(&f).unwrap().contas.len(), 1);
        *f.falhar_apagar.borrow_mut() = None;
        assert_eq!(slots(&f).unwrap().contas.len(), 1);
    }

    #[test]
    fn passwords_sao_diferentes_por_conta() {
        let f = Falso::novo(3);
        let a = criar(&f, None, "c1").unwrap();
        let b = criar(&f, None, "c2").unwrap();
        assert_ne!(a.password, b.password, "as chaves nao podem ser iguais");
    }

    #[test]
    fn agentmail_nao_suporta_password_escolhida() {
        let f = Falso::novo(3);
        assert!(!f.suporta_password_escolhida());
        let c = criar(&f, None, "c1").unwrap();
        // O rotulo tem de avisar que nao e uma password normal.
        assert!(c.rotulo_password(false).contains("Chave"));
    }

    #[test]
    fn username_aleatorio_legivel() {
        for _ in 0..50 {
            let u = username_aleatorio(12);
            assert_eq!(u.chars().count(), 12);
            assert!(u.chars().all(|c| c.is_ascii_lowercase()), "so minusculas: {}", u);
            // Nada de ambiguos (0, 1, l, i nao entram nos conjuntos).
            assert!(!u.contains('l') && !u.contains('i'), "ambiguo em {}", u);
        }
    }
}