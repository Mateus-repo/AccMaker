//! Medicao de tamanho: pedido HTTPS com ureq + rustls.
//!
//! Traz o stack TLS inteiro para dentro do binario. Apenas para medir.

fn principal() {
    let resposta = ureq::post("https://purelymail.com/api/v0/listDomains")
        // Token de exemplo, sem valor real.
        .header("Purelymail-Api-Token", "<API_TOKEN>")
        .header("Content-Type", "application/json")
        .send(r#"{"includeShared":true}"#);

    match resposta {
        Ok(r) => {
            let estado = r.status().as_u16();
            println!("pedido HTTPS feito: estado {}", estado);
        }
        Err(e) => println!("erro ao pedir: {}", e),
    }
}

fn main() {
    principal();
}