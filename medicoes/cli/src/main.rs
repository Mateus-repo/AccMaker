//! Medição de tamanho: variante CLI, sem interface gráfica.
// Escreve uma password de exemplo e sai. Serve apenas para medir o .exe.

fn main() {
    let saida = format!("medicao-cli {}", env!("CARGO_PKG_VERSION"));
    println!("{}", saida);
}