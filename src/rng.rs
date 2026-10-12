//! Acesso ao gerador aleatorio do sistema operativo.
//!
//! `getrandom` e a interface do sistema (BCryptGenRandom no Windows,
//! getrandom(2) em Linux, getentropy(2) em macOS). Nao ha seed, nao ha estado
//! global e a sequencia nao se deduz do tempo.

/// Gerador de bytes do sistema.
pub struct OsRng;

impl OsRng {
    /// Devolve um byte aleatorio do sistema.
    pub fn next_u8(&mut self) -> u8 {
        let mut b = [0u8; 1];
        self.fill(&mut b);
        b[0]
    }

    /// Preenche um buffer com bytes aleatorios do sistema.
    pub fn fill(&mut self, destino: &mut [u8]) {
        getrandom::getrandom(destino).expect("o sistema nao gerou bytes aleatorios");
    }
}