//! AccMaker — gerador de passwords.
//!
//! Fase 1: so o gerador, sem rede. O cliente da API entra mais tarde, atras de
//! uma interface `provider`.

mod password;
mod rng;

pub use password::{
    caracteres_disponiveis, classes_presentes, forca_estimada, gerar, gerar_limitado,
    verificar, Erro, Opcoes, TAMANHO_MAXIMO_PRUDENTE,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut opcoes = Opcoes::default();
    if args.iter().any(|a| a == "--sem-minusculas") {
        opcoes.minusculas = false;
    }
    if args.iter().any(|a| a == "--sem-maiusculas") {
        opcoes.maiusculas = false;
    }
    if args.iter().any(|a| a == "--sem-numeros") {
        opcoes.numeros = false;
    }
    if args.iter().any(|a| a == "--sem-simbolos") {
        opcoes.simbolos = false;
    }
    if args.iter().any(|a| a == "--ambiguos") {
        opcoes.excluir_ambiguos = false;
    }

    if args.iter().any(|a| a == "--caracteres") {
        println!("{}", password::caracteres_disponiveis(&opcoes));
        return;
    }

    let tamanho = args
        .iter()
        .find_map(|a| a.strip_prefix("--tamanho="))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(16);

    // Enquanto o limite oficial do fornecedor nao se sabe, o teto e prudente.
    let teto = match args.iter().find_map(|a| a.strip_prefix("--teto=")) {
        Some(v) => v.parse::<usize>().ok().filter(|n| *n > 0),
        None => Some(TAMANHO_MAXIMO_PRUDENTE),
    };

    if tamanho == 0 {
        eprintln!("erro: o tamanho tem de ser pelo menos 1");
        std::process::exit(2);
    }

    let quantidade = args
        .iter()
        .find_map(|a| a.strip_prefix("--quantidade="))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1);

    for _ in 0..quantidade {
        match password::gerar_limitado(tamanho, opcoes, teto) {
            Ok(p) => {
                // Reconfirma as garantias antes de mostrar ao utilizador.
                if let Err(motivo) = password::verificar(&p, &opcoes) {
                    eprintln!("erro interno: a password gerada nao cumpre as regras: {}", motivo);
                    std::process::exit(3);
                }
                println!("{}", p);
            }
            Err(e) => {
                eprintln!("erro: {}", e);
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use password::classes_presentes;
    use std::collections::HashSet;

    const VEZES: usize = 400;

    fn opcoes_padrao() -> Opcoes {
        Opcoes::default()
    }

    #[test]
    fn comprimento_exato() {
        for n in [4usize, 8, 12, 16, 32, 64] {
            let p = password::gerar(n, opcoes_padrao()).unwrap();
            assert_eq!(p.chars().count(), n, "comprimento errado para {}", n);
        }
    }

    #[test]
    fn nunca_fica_sem_caracter_de_alguma_classe() {
        let opcoes = opcoes_padrao();
        for _ in 0..VEZES {
            let p = password::gerar(4, opcoes).unwrap();
            let c = classes_presentes(&p);
            assert_eq!(c.len(), 4, "faltou classe em {:?}", p);
            assert!(c.contains("minuscula"), "sem minuscula em {:?}", p);
            assert!(c.contains("maiuscula"), "sem maiuscula em {:?}", p);
            assert!(c.contains("numero"), "sem numero em {:?}", p);
            assert!(c.contains("simbolo"), "sem simbolo em {:?}", p);
        }
    }

    #[test]
    fn nunca_deixa_caracter_ambiguo() {
        let opcoes = opcoes_padrao();
        assert!(opcoes.excluir_ambiguos);
        for _ in 0..VEZES {
            let p = password::gerar(16, opcoes).unwrap();
            for ch in p.chars() {
                assert!(
                    !"0O1lI".contains(ch),
                    "caracter ambiguo {:?} em {}",
                    ch,
                    p
                );
            }
            assert!(!p.contains(' '), "espaco em {}", p);
        }
    }

    #[test]
    fn modo_ambiguos_ligado_tem_mais_opcoes() {
        let sem = password::caracteres_disponiveis(&Opcoes {
            excluir_ambiguos: true,
            ..Opcoes::default()
        });
        let com = password::caracteres_disponiveis(&Opcoes {
            excluir_ambiguos: false,
            ..Opcoes::default()
        });
        assert!(
            com.len() > sem.len(),
            "o modo ambiguo devia aumentar o alfabeto: {} vs {}",
            sem.len(),
            com.len()
        );
    }

    #[test]
    fn modo_ambiguos_ligado_aceita_ambiguos() {
        let opcoes = Opcoes {
            excluir_ambiguos: false,
            ..Opcoes::default()
        };
        let mut apareceu_ambiguo = false;
        for _ in 0..VEZES {
            let p = password::gerar(24, opcoes).unwrap();
            if p.chars().any(|c| "0O1lI".contains(c)) {
                apareceu_ambiguo = true;
                break;
            }
        }
        assert!(
            apareceu_ambiguo,
            "com ambiguos ligados nunca saiu um caracter ambiguo"
        );
        // Nunca espacos nem aspas, que partiriam a linha ou o JSON.
        for _ in 0..VEZES {
            let p = password::gerar(24, opcoes).unwrap();
            assert!(!p.contains(' '));
            assert!(!p.contains('"'));
        }
    }

    #[test]
    fn classes_desligadas_respeitam_a_opcao() {
        let opcoes = Opcoes {
            numeros: false,
            ..Opcoes::default()
        };
        for _ in 0..VEZES {
            let p = password::gerar(8, opcoes).unwrap();
            assert!(
                !p.chars().any(|c| c.is_ascii_digit()),
                "numero presente com numeros desligados: {}",
                p
            );
        }
    }

    #[test]
    fn comprimento_menino_que_o_numero_de_classes_da_erro() {
        let opcoes = opcoes_padrao();
        assert_eq!(
            password::gerar(3, opcoes),
            Err(Erro::CurtoDemais { pedido: 3, minimo: 4 })
        );
    }

    #[test]
    fn sem_nenhuma_classe_da_erro() {
        let opcoes = Opcoes {
            minusculas: false,
            maiusculas: false,
            numeros: false,
            simbolos: false,
            excluir_ambiguos: true,
        };
        assert_eq!(password::gerar(16, opcoes), Err(Erro::SemClasses));
    }

    #[test]
    fn nunca_repete_o_mesmo_valor_consecutivo() {
        let opcoes = opcoes_padrao();
        let mut anterior = password::gerar(16, opcoes).unwrap();
        for _ in 0..50 {
            let atual = password::gerar(16, opcoes).unwrap();
            assert_ne!(anterior, atual, "o gerador devolveu duas iguais seguidas");
            anterior = atual;
        }
    }

    #[test]
    fn valores_sao_variados() {
        // Duas passwords de 16 chars num alfabeto de ~80 deviam ser distintas;
        // 200 amostras iguais apontariam para um gerador avariado.
        let opcoes = opcoes_padrao();
        let vistas: HashSet<String> = (0..200).map(|_| password::gerar(16, opcoes).unwrap()).collect();
        assert_eq!(vistas.len(), 200, "demasiadas passwords iguais");
    }

    #[test]
    fn tetos_sao_respeitados() {
        let opcoes = opcoes_padrao();
        // Fornecedor com limite abaixo do pedido.
        let p = password::gerar_limitado(64, opcoes, Some(12)).unwrap();
        assert_eq!(p.chars().count(), 12);
        // Sem limite do fornecedor aplica o teto prudente.
        let p = password::gerar_limitado(500, opcoes, None).unwrap();
        assert_eq!(p.chars().count(), TAMANHO_MAXIMO_PRUDENTE);
        // Teto prudente aplica-se mesmo com limite maior do fornecedor.
        let p = password::gerar_limitado(500, opcoes, Some(1000)).unwrap();
        assert_eq!(p.chars().count(), TAMANHO_MAXIMO_PRUDENTE);
    }

    #[test]
    fn verifica_deteta_password_invalida() {
        let opcoes = opcoes_padrao();
        assert!(password::verificar("abc", &opcoes).is_err());
        assert!(password::verificar("Ab3!", &opcoes).is_ok());
        assert!(password::verificar("Abcdefgh3!", &opcoes).is_ok());
        // Com ambiguos, um deles estraga a password.
        assert!(password::verificar("Ab1!", &opcoes).is_err(), "1 e ambiguo");
    }

    #[test]
    fn forca_cresce_com_o_tamanho() {
        let opcoes = opcoes_padrao();
        let curta = password::forca_estimada(8, &opcoes);
        let longa = password::forca_estimada(32, &opcoes);
        assert!(longa > curta);
        assert!(curta > 0.0);
    }
}