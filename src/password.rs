//! Gerador de passwords do AccMaker.
//!
//! Usa o gerador aleatório do sistema (`getrandom`, via `OsRng`), nunca `rand`.
//! A escolha de caracteres é feita por rejeição, para não introduzir
//! enviesamento de módulo.

use std::collections::HashSet;

/// Conjuntos completos por classe, antes de qualquer exclusao.
/// Incluem os caracteres ambiguos, para que o filtro tenha o que remover.
const MINUSCULAS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const MAIUSCULAS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMEROS: &[u8] = b"0123456789";
const SIMBOLOS: &[u8] = b"!@#$%^&*()-_=+?";

/// Caracteres ambíguos, excluídos quando `excluir_ambiguos` esta ligado.
const AMBIGUOS: &[u8] = b"0O1lI|`'\"\\;,.:/ ";

/// Classes que podem ser usadas numa password.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opcoes {
    pub minusculas: bool,
    pub maiusculas: bool,
    pub numeros: bool,
    pub simbolos: bool,
    /// Quando verdadeiro, retira os caracteres ambíguos.
    pub excluir_ambiguos: bool,
}

impl Default for Opcoes {
    fn default() -> Self {
        Opcoes {
            minusculas: true,
            maiusculas: true,
            numeros: true,
            simbolos: true,
            excluir_ambiguos: true,
        }
    }
}

/// Erros do gerador.
#[derive(Debug, PartialEq, Eq)]
pub enum Erro {
    /// O comprimento pedido é menor do que o número de classes ativas.
    CurtoDemais { pedido: usize, minimo: usize },
    /// Nenhuma classe de caracteres está ativa.
    SemClasses,
}

impl std::fmt::Display for Erro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Erro::CurtoDemais { pedido, minimo } => write!(
                f,
                "comprimento {} e menor do que o minimo {} para as classes ativas",
                pedido, minimo
            ),
            Erro::SemClasses => write!(f, "nenhuma classe de caracteres ativa"),
        }
    }
}

/// Conjunto efetivo de caracteres, já sem os ambíguos.
fn conjunto(opcoes: &Opcoes) -> Vec<u8> {
    let mut todos: Vec<u8> = Vec::new();
    let juntar = |lista: &'static [u8], todos: &mut Vec<u8>| {
        todos.extend_from_slice(lista);
    };
    if opcoes.minusculas {
        juntar(MINUSCULAS, &mut todos);
    }
    if opcoes.maiusculas {
        juntar(MAIUSCULAS, &mut todos);
    }
    if opcoes.numeros {
        juntar(NUMEROS, &mut todos);
    }
    if opcoes.simbolos {
        juntar(SIMBOLOS, &mut todos);
    }
    if opcoes.excluir_ambiguos {
        todos.retain(|c| !AMBIGUOS.contains(c));
    } else {
        // Mesmo sem modo restrito, nunca devolver espacos nem aspas, que
        // partiriam a linha na apresentacao e o JSON.
        todos.retain(|c| !matches!(c, b' ' | b'"' | b'\\'));
    }
    todos.sort_unstable();
    todos.dedup();
    todos
}

/// As classes ativas, como listas de caracteres.
fn classes_ativas(opcoes: &Opcoes) -> Vec<Vec<u8>> {
    let mut lista: Vec<Vec<u8>> = Vec::new();
    for (ativo, fonte) in [
        (opcoes.minusculas, MINUSCULAS),
        (opcoes.maiusculas, MAIUSCULAS),
        (opcoes.numeros, NUMEROS),
        (opcoes.simbolos, SIMBOLOS),
    ] {
        if !ativo {
            continue;
        }
        let mut c: Vec<u8> = fonte.to_vec();
        if opcoes.excluir_ambiguos {
            c.retain(|x| !AMBIGUOS.contains(x));
        }
        lista.push(c);
    }
    lista
}

/// Escolhe um indice uniformemente em `0..n`, sem enviesamento de modulo.
///
/// Um byte tem 256 valores. Se 256 nao for multiplo de `n`, os primeiros
/// `256 % n` indices sairiam mais vezes do que os outros. Para evitar isso,
/// rejeita-se a cauda da tabela — de `n * (256 / n)` ate 255 — e so se aceita
/// o que sobrar, que ja divide certo por `n`.
fn indice_sem_enviesamento(rng: &mut dyn FnMut() -> u8, n: usize) -> usize {
    assert!(n > 0);
    debug_assert!(
        n <= 256,
        "o gerador de um byte nao cobre um alfabeto de {} caracteres",
        n
    );
    let limite = (256 / n) * n; // maior multiplo de n dentro de 0..256
    loop {
        let v = rng() as usize;
        if v < limite {
            return v % n;
        }
    }
}

/// Gera uma password com as opcoes dadas e o comprimento pedido.
///
/// Garante pelo menos um caracter de cada classe ativa, desde que o comprimento
/// chegue para isso (validado antes, em `Erro::CurtoDemais`).
pub fn gerar(tamanho: usize, opcoes: Opcoes) -> Result<String, Erro> {
    let mut rng = crate::rng::OsRng;

    let todos = conjunto(&opcoes);
    if todos.is_empty() {
        return Err(Erro::SemClasses);
    }
    let classes = classes_ativas(&opcoes);
    let minimo = classes.len();
    if tamanho < minimo {
        return Err(Erro::CurtoDemais {
            pedido: tamanho,
            minimo,
        });
    }

    // Um caracter garantido de cada classe ativa.
    let mut saida: Vec<u8> = Vec::with_capacity(tamanho);
    for classe in &classes {
        saida.push(classe[indice_sem_enviesamento(&mut || rng.next_u8(), classe.len())]);
    }

    // O resto vem do conjunto completo.
    while saida.len() < tamanho {
        saida.push(todos[indice_sem_enviesamento(&mut || rng.next_u8(), todos.len())]);
    }

    // Baralhar com Fisher-Yates para que a posicao dos caracteres garantidos
    // nao denuncie a estrutura da password.
    for i in (1..saida.len()).rev() {
        let j = indice_sem_enviesamento(&mut || rng.next_u8(), i + 1);
        saida.swap(i, j);
    }

    // Converter para String sem perder os simbolos ASCII escolhidos.
    Ok(saida.iter().map(|&b| b as char).collect())
}

/// Tamanho maximo prudente enquanto o limite oficial do fornecedor nao se sabe.
pub const TAMANHO_MAXIMO_PRUDENTE: usize = 64;

/// Gera uma password dentro dos limites validos, limitando pelo fornecedor.
pub fn gerar_limitado(
    tamanho: usize,
    opcoes: Opcoes,
    maximo_do_fornecedor: Option<usize>,
) -> Result<String, Erro> {
    let teto = match maximo_do_fornecedor {
        Some(m) if m > 0 => m.min(TAMANHO_MAXIMO_PRUDENTE),
        _ => TAMANHO_MAXIMO_PRUDENTE,
    };
    let tamanho = tamanho.min(teto);
    gerar(tamanho, opcoes)
}

/// Verifica que a password cumpre as garantias do gerador.
pub fn verificar(password: &str, opcoes: &Opcoes) -> Result<(), String> {
    let chars: Vec<u8> = password.bytes().collect();
    if chars.is_empty() {
        return Err("password vazia".into());
    }
    if opcoes.excluir_ambiguos {
        if let Some(c) = chars.iter().find(|c| AMBIGUOS.contains(c)) {
            return Err(format!("contem caracter ambiguo {:?}", *c as char));
        }
    }
    let tem = |lista: &[u8]| chars.iter().any(|c| lista.contains(c));
    let mut em_falta: Vec<&str> = Vec::new();
    if opcoes.minusculas && !tem(MINUSCULAS) {
        em_falta.push("minuscula");
    }
    if opcoes.maiusculas && !tem(MAIUSCULAS) {
        em_falta.push("maiuscula");
    }
    if opcoes.numeros && !tem(NUMEROS) {
        em_falta.push("numero");
    }
    if opcoes.simbolos && !tem(SIMBOLOS) {
        em_falta.push("simbolo");
    }
    if !em_falta.is_empty() {
        return Err(format!("faltam classes: {}", em_falta.join(", ")));
    }
    Ok(())
}

/// Quantas passwords distintas o conjunto efetivo permite, paraAvaliar a forca.
pub fn forca_estimada(tamanho: usize, opcoes: &Opcoes) -> f64 {
    let n = conjunto(opcoes).len() as f64;
    if n <= 1.0 {
        return 0.0;
    }
    tamanho as f64 * n.log2()
}

/// Lista de caracteres efetivamente usados, para mostrar na interface.
pub fn caracteres_disponiveis(opcoes: &Opcoes) -> String {
    conjunto(opcoes).iter().map(|&b| b as char).collect()
}

/// Contagem de classes distintas numa password, usada nos testes.
pub fn classes_presentes(password: &str) -> HashSet<&'static str> {
    let mut found = HashSet::new();
    for c in password.chars() {
        let b = c as u8;
        if MINUSCULAS.contains(&b) {
            found.insert("minuscula");
        } else if MAIUSCULAS.contains(&b) {
            found.insert("maiuscula");
        } else if NUMEROS.contains(&b) {
            found.insert("numero");
        } else if SIMBOLOS.contains(&b) {
            found.insert("simbolo");
        }
    }
    found
}