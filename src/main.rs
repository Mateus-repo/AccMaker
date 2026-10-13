//! AccMaker — CLI.
//!
//! Comandos: `listar`, `criar`, `apagar`, `trocar`, `password`.
//! Todos aceitam `--dry-run` para mostrar o pedido sem o enviar.

mod agentmail;
mod cofre;
mod janela;
mod password;
mod provider;
mod rng;
mod slots;

#[cfg(test)]
mod cli_testes;

pub use agentmail::AgentMail;
pub use cofre::{mascarar, Cofre, CofreFalso, CofreWindows};
pub use password::{
    caracteres_disponiveis, classes_presentes, forca_estimada, gerar, gerar_limitado,
    verificar, Erro as ErroPassword, Opcoes, TAMANHO_MAXIMO_PRUDENTE,
};
pub use provider::{Capacidade, Credenciais, Erro, Fornecedor, Inbox};
pub use slots::{apagar, criar, slots, trocar, username_aleatorio, Slots};

/// Valor sentinela para quando o utilizador diz "ao acaso".
const QUALQUER: &str = "?";

/// As opcoes da linha de comandos.
struct Args {
    comando: String,
    username: Option<String>,
    inbox_id: Option<String>,
    dry_run: bool,
    confirmar: bool,
    tamanho: usize,
    quantidade: usize,
    opcoes: Opcoes,
    caracteres: bool,
}

fn analisar(argumentos: &[String]) -> Result<Args, String> {
    let mut comando = String::new();
    let mut inbox_id: Option<String> = None;
    let mut posicionais: Vec<String> = Vec::new();

    for a in argumentos {
        if a.starts_with("--") {
            continue;
        }
        if comando.is_empty() {
            comando = a.clone();
        } else if inbox_id.is_none() {
            inbox_id = Some(a.clone());
        } else {
            posicionais.push(a.clone());
        }
    }
    let _ = posicionais;

    let tem = |nome: &str| argumentos.iter().any(|a| a == nome);
    let valor = |nome: &str| {
        argumentos
            .iter()
            .find_map(|a| a.strip_prefix(&format!("{}=", nome)))
            .map(|v| v.to_string())
    };

    let tamanho = valor("--tamanho")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(16);
    if tamanho == 0 {
        return Err("o tamanho tem de ser pelo menos 1".into());
    }

    Ok(Args {
        comando,
        username: match valor("--username").as_deref() {
            None | Some(QUALQUER) => None,
            Some(u) if u.is_empty() => None,
            Some(u) => Some(u.to_string()),
        },
        inbox_id,
        dry_run: tem("--dry-run"),
        confirmar: tem("--yes") || tem("--sim"),
        tamanho,
        quantidade: valor("--quantidade")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(1),
        opcoes: Opcoes {
            minusculas: !tem("--sem-minusculas"),
            maiusculas: !tem("--sem-maiusculas"),
            numeros: !tem("--sem-numeros"),
            simbolos: !tem("--sem-simbolos"),
            excluir_ambiguos: !tem("--ambiguos"),
        },
        caracteres: tem("--caracteres"),
    })
}

/// `client_id` unico por tentativa, para repetir o pedido sem duplicar.
fn novo_client_id() -> String {
    let mut bytes = [0u8; 16];
    let mut rng = rng::OsRng;
    rng.fill(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Pede confirmacao ao utilizador antes de uma accao destrutiva ou que gasta
/// uma das 3 vagas. Com `--yes` nao pergunta.
fn confirmar(a: &Args, pergunta: &str) -> bool {
    if a.confirmar || a.dry_run {
        return true;
    }
    print!("{} (s/N) ", pergunta);
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let mut resposta = String::new();
    if std::io::stdin().read_line(&mut resposta).is_err() {
        return false;
    }
    matches!(resposta.trim().to_lowercase().as_str(), "s" | "sim" | "y" | "yes")
}

fn mostrar_credenciais(c: &Credenciais, password_escolhida: bool) {
    println!();
    println!("  Email:  {}", c.email);
    println!("  {}:  {}", c.rotulo_password(password_escolhida), c.password);
    println!();
}

fn main() {
    let argumentos: Vec<String> = std::env::args().skip(1).collect();
    let a = match analisar(&argumentos) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("erro: {}", e);
            std::process::exit(2);
        }
    };

    // Gerador de passwords: nao precisa de rede nem de chave.
    if a.comando == "password" {
        if a.caracteres {
            println!("{}", caracteres_disponiveis(&a.opcoes));
            return;
        }
        for _ in 0..a.quantidade {
            match gerar_limitado(a.tamanho, a.opcoes, None) {
                Ok(p) => println!("{}", p),
                Err(e) => {
                    eprintln!("erro: {}", e);
                    std::process::exit(1);
                }
            }
        }
        return;
    }

    if a.comando == "interface" {
        janela::abrir("accmaker");
        return;
    }

    if a.comando.is_empty() || a.comando == "ajuda" || a.comando == "--ajuda" {
        ajuda();
        return;
    }

    // `testar-chave` usa uma key restrita, não a de organização. Pedir a de
    // organização aqui seria exigir a key errada.
    if a.comando == "testar-chave" {
        match cmd_testar_chave(&a) {
            Ok(()) => return,
            Err(e) => {
                eprintln!("erro: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Daqui para baixo e preciso da API key.
    let cofre = CofreWindows;
    let chave = match cofre::ler_api_key(&cofre) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("erro: {}", e);
            std::process::exit(2);
        }
    };

    let f: AgentMail = if a.dry_run {
        AgentMail::dry_run(chave)
    } else {
        AgentMail::novo(chave)
    };

    let resultado = match a.comando.as_str() {
        "listar" => cmd_listar(&f),
        "criar" => cmd_criar(&f, &a, &cofre),
        "apagar" => cmd_apagar(&f, &a),
        "trocar" => cmd_trocar(&f, &a),
        "testar-chave" => unreachable!("tratado antes de pedir a key de organizacao"),
        "criar-chave" => cmd_criar_chave(&a),
        outro => Err(format!(
            "comando desconhecido: {}. Vê `accmaker ajuda`.",
            outro
        )),
    };

    match resultado {
        Ok(()) => {}
        Err(e) => {
            eprintln!("erro: {}", e);
            std::process::exit(1);
        }
    }
}

/// Teste da pergunta 6: uma key restrita a uma inbox consegue apagar essa
/// mesma inbox?
///
/// A key entra pela variavel `ACCMAILER_CHAVE_TESTE`, que o utilizador define
/// com `Read-Host` para nao ficar no historico. Nunca e impressa.
fn cmd_testar_chave(a: &Args) -> Result<(), String> {
    let id = match &a.inbox_id {
        Some(i) => i.clone(),
        None => return Err("falta o id da inbox.".into()),
    };
    let chave = match std::env::var("ACCMAILER_CHAVE_TESTE") {
        Ok(k) if !k.trim().is_empty() => k,
        _ => {
            return Err(
                "Define primeiro ACCMAILER_CHAVE_TESTE com a key restrita DESSA inbox:\n  \
                 $env:ACCMAILER_CHAVE_TESTE = Read-Host 'Chave restrita da inbox'"
                    .into(),
            )
        }
    };

    let cliente = agentmail::com_chave(chave);
    println!("Inbox: {}", id);
    println!("A testar o que a key restrita dela consegue fazer...");
    println!();

    let passos = cliente.apagar_com_key(&id);
    let mut ultimo: Option<&agentmail::Passo> = None;
    for p in &passos {
        println!("[{}] {}", p.nome, p.resposta.resumo());
        ultimo = Some(p);
    }
    let fim = match ultimo {
        Some(p) => p.resposta.estado,
        None => 0,
    };
    println!();
    println!("INTERPRETACAO");
    println!();

    if fim == 403 || fim == 401 {
        println!("A key restrita NAO consegue apagar a inbox.");
        println!("A API recusou o pedido. Quem tenha a key nao consegue");
        println!("destruir a conta, por isso mostra-la no ecra e aceitavel.");
    } else if (200..300).contains(&fim) {
        println!("A key restrita APAGOU a inbox.");
        println!("Isto e um risco: quem tenha a key consegue destruir a conta.");
        println!("A janela nao deve mostrar a key em claro, so um botao copiar.");
    } else if passos.iter().any(|p| p.resposta.estado == 409) {
        println!("A inbox tem keys em cima e nao se deixou apagar.");
        println!("O 409 nao prova nada: so diz que houve conflito.");
        if let Some(p) = passos.iter().find(|p| p.nome.contains("DELETE de uma key")) {
            println!();
            println!("Ao tentar apagar as keys com a propria key restrita:");
            println!("  {}", p.resposta.resumo());
            println!();
            if p.resposta.estado == 403 || p.resposta.estado == 401 {
                println!("A key NAO apaga as keys. Logo NAO consegue destruir a conta.");
                println!("Mostrar a key no ecra e aceitavel.");
            } else {
                println!("A key mexe nas keys da inbox. O DELETE final deu {}:", fim);
                println!("consulta os passos acima para o codigo exacto.");
            }
        } else {
            println!("Nao foi possivel nem listar as keys da inbox com esta key.");
            println!("A key parece ter poucas permissoes. Trata como \"nao apaga\".");
        }
    } else {
        println!("Resultado inconclusivo (estado final {}).", fim);
        println!("Confirma o estado com `accmaker listar`.");
    }
    println!();
    println!("A key nao foi impressa nem registada.");
    Ok(())
}

/// Teste da pergunta 7: da para criar uma segunda key para uma inbox que ja
/// existe? Se sim, uma conta nunca fica sem chave.
fn cmd_criar_chave(a: &Args) -> Result<(), String> {
    let id = match &a.inbox_id {
        Some(i) => i.clone(),
        None => return Err("falta o id da inbox.".into()),
    };
    if !confirmar(a, "Criar uma key nova para essa inbox?") {
        return Ok(());
    }
    let f = agentmail::com_chave(ler_key_organizacao()?);
    let caminho = format!("/v0/inboxes/{}/api-keys", id);
    let corpo = r#"{"name":"accmaker-extra"}"#;
    match f.criar_key(&caminho, corpo) {
        Ok(chave) => {
            println!();
            println!("RESULTADO: deu para criar outra key para a mesma inbox.");
            println!("Guarda-a: e a resposta a pergunta 7.");
            println!();
            println!("  Chave:  {}", chave);
            println!();
            println!("Nao a partilhes no chat.");
        }
        Err(e) => {
            println!();
            println!("RESULTADO: nao deu para criar outra key ({}).", e);
        }
    }
    Ok(())
}

fn ler_key_organizacao() -> Result<String, String> {
    let cofre = CofreWindows;
    cofre::ler_api_key(&cofre)
}

fn cmd_listar(f: &AgentMail) -> Result<(), String> {
    let s = slots(f).map_err(|e| e.to_string())?;
    if s.contas.is_empty() {
        println!("Nenhuma conta criada.");
    } else {
        for (i, inbox) in s.contas.iter().enumerate() {
            println!("{}. {}  ({}  {})", i + 1, inbox.email, inbox.id, inbox.criada_em);
        }
    }
    println!();
    println!("{}/{} contas ocupadas.", s.capacidade.usadas, s.capacidade.maximo);
    Ok(())
}

/// Resolva a conta orfa: a inbox foi criada mas a chave nao ficou guardada.
///
/// A regra depende de haver alguem a ver o ecra:
/// - com `--yes` nao ha ninguem, e a key mostrada vai ser perdida. Apaga-se a
///   inbox, para nao deixar uma conta inutilizavel.
/// - sem `--yes` pergunta-se. Quem decide e o utilizador: a inbox nova e util
///   (recebe email) mesmo sem a key guardada.
fn resolver_orfa(f: &AgentMail, a: &Args, cred: &Credenciais, inbox_id: &str) -> Result<(), String> {
    println!();
    println!("ATENCAO: a conta foi criada mas a chave NAO ficou guardada.");
    println!("No AgentMail a chave e a password. Sem ela, nao se entra na conta.");
    println!();
    println!("  Email:  {}", cred.email);
    println!("  Chave:  {}", cred.password);
    println!();

    if a.confirmar {
        // Ninguem vai ler o ecra. Apagar e o que evita deixar uma conta morta.
        println!("Sem confirmacao interactiva: a apagar a inbox orfa.");
        f.apagar(inbox_id).map_err(|e| e.to_string())?;
        println!("Inbox apagada. A vaga ficou livre.");
        return Ok(());
    }

    if perguntar("Ja guardaste a chave?") {
        println!("A inbox fica. Guarda a chave: sem ela a conta nao serve.");
    } else {
        println!("Vou apagar a inbox.");
        f.apagar(inbox_id).map_err(|e| e.to_string())?;
        println!("Inbox apagada. A vaga ficou livre.");
    }
    Ok(())
}

/// Pergunta ao utilizador. Sem `--yes`, le do ecra.
fn perguntar(texto: &str) -> bool {
    print!("{} (s/N) ", texto);
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let mut resposta = String::new();
    if std::io::stdin().read_line(&mut resposta).is_err() {
        return false;
    }
    matches!(
        resposta.trim().to_lowercase().as_str(),
        "s" | "sim" | "y" | "yes"
    )
}

fn cmd_criar(f: &AgentMail, a: &Args, cofre: &dyn Cofre) -> Result<(), String> {
    // A listagem ja happened antes para confirmar; em dry-run nao vale a pena
    // repetir o pedido so para contar as vagas.
    let c = if a.dry_run {
        slots::Slots {
            contas: Vec::new(),
            capacidade: Capacidade { usadas: 0, maximo: 3 },
        }
    } else {
        slots(f).map_err(|e| e.to_string())?
    };
    if c.capacidade.cheia() {
        return Err(format!(
            "Chegaste ao limite de {} contas. Apaga uma com `accmaker apagar <id>`.",
            c.capacidade.maximo
        ));
    }
    let client_id = novo_client_id();
    let username = a
        .username
        .clone()
        .unwrap_or_else(|| username_aleatorio(12));
    println!("Username: {}", username);

    if !confirmar(a, "Criar conta nova?") {
        return Ok(());
    }

    let cred = criar(f, Some(&username), &client_id).map_err(|e| e.to_string())?;

    // Gravar a key da conta no cofre. Se falhar, a conta fica orfa: mostrar a
    // key uma vez e avisar, em vez de dizer que correu bem.
    let nome_cofre = format!("inbox-{}", &cred.email);
    // Em dry-run nada e gravado: seria estragar o Credential Manager com uma
    // conta que nao existe.
    if a.dry_run {
        mostrar_credenciais(&cred, false);
        println!("(dry-run: nada foi gravado no Credential Manager)");
        return Ok(());
    }

    match cofre.gravar(&nome_cofre, &cred.password) {
        Ok(()) => {
            mostrar_credenciais(&cred, false);
            println!("Chave guardada no Credential Manager ({}).", nome_cofre);
        }
        Err(e) => {
            println!("Motivo da falha: {}", e);
            let id = f
                .ultimo_id()
                .unwrap_or_else(|| cred.email.clone());
            resolver_orfa(f, a, &cred, &id)?;
        }
    }
    Ok(())
}

fn cmd_apagar(f: &AgentMail, a: &Args) -> Result<(), String> {
    let id = match &a.inbox_id {
        Some(i) => i.clone(),
        None => return Err("falta o id da conta. Vê `accmaker listar`.".into()),
    };
    if !confirmar(
        a,
        "Isto apaga a conta e TODAS as mensagens. Nao da para anular. Continuar?",
    ) {
        return Ok(());
    }
    let s = apagar(f, &id).map_err(|e| e.to_string())?;
    println!("Conta apagada.");
    println!(
        "{}/{} contas ocupadas.",
        s.capacidade.usadas, s.capacidade.maximo
    );
    Ok(())
}

fn cmd_trocar(f: &AgentMail, a: &Args) -> Result<(), String> {
    let id = match &a.inbox_id {
        Some(i) => i.clone(),
        None => return Err("falta o id da conta a trocar. Vê `accmaker listar`.".into()),
    };
    if !confirmar(
        a,
        "Trocar apaga a conta antiga e TODAS as suas mensagens, e cria uma nova. Continuar?",
    ) {
        return Ok(());
    }
    let client_id = novo_client_id();
    let username = a
        .username
        .clone()
        .unwrap_or_else(|| username_aleatorio(12));
    println!("Username novo: {}", username);
    let cred = trocar(f, &id, Some(&username), &client_id).map_err(|e| e.to_string())?;
    mostrar_credenciais(&cred, false);
    println!("Conta trocada.");
    Ok(())
}

fn ajuda() {
    println!("accmaker — criar e trocar contas de email");
    println!();
    println!("COMANDOS");
    println!("  listar                 mostra as contas e as vagas");
    println!("  criar                  cria uma conta nova (pede confirmacao)");
    println!("  apagar <id>            apaga uma conta");
    println!("  trocar <id>            apaga uma conta e cria outra");
    println!("  interface              abre a janela");
    println!("  password               gera passwords (sem rede)");
    println!("  testar-chave <id>      a key restrita apaga a propria inbox?");
    println!("  criar-chave <id>       cria outra key para a mesma inbox");
    println!("  ajuda                  esta ajuda");
    println!();
    println!("OPCIONES");
    println!("  --dry-run              mostra o pedido HTTP sem o enviar");
    println!("  --yes                  nao pedir confirmacao");
    println!("  --username=<nome>      escolhe o username; omissao = ao acaso");
    println!("  --tamanho=<n>          tamanho da password (comando password)");
    println!("  --quantidade=<n>       quantas passwords (comando password)");
    println!();
    println!("API KEY");
    println!("  De AGENTMAIL_API_KEY ou do Credential Manager.");
    println!("  Nunca em argumento de linha de comandos.");
    println!();
    println!("  No PowerShell, prefere nao deixar a key no historico:");
    println!("    $env:AGENTMAIL_API_KEY = Read-Host 'AgenteMail API key'");
}