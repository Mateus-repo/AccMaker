# AccMaker

Cria e troca contas de email a partir de um **único `.exe` leve**, sem instalação.

Tens **3 inboxes grátis**. O AccMaker cria uma, mostra o email e a chave de acesso, e
deixa-te apagar uma conta e criar outra em cima. É a operação de "trocar de conta", o
que o torna útil para isso que normalmente demora dez minutos.

## O que faz

- **Criar** uma conta: dá o email e a chave de acesso, prontos a copiar.
- **Listar** as contas em uso e as vagas livres.
- **Apagar** uma conta, com confirmação.
- **Trocar**: apagar uma e criar outra numa acção só, sem deixar a vaga vazia se algo
  correr mal a meio.
- **Gerador de passwords** com o gerador aleatório do sistema (sem rede), para
  fornecedores em que a password é escolhida por ti.

## A "password" é uma chave de acesso

Este é o ponto que surpreende, e é o mais importante do AccMaker:

**O AgentMail não tem passwords.** A password do IMAP e do SMTP de cada conta é uma
**API key**. O AccMaker cria uma key só para cada conta e mostra-a.

Três coisas seguem daí, todas verificadas com uma conta a sério:

1. **A chave pode aparecer no ecrã.** Quem a tiver não consegue apagar a conta: a API
   responde `403 missing_permission`. Não há risco de alguém destruir a conta por ter
   a chave.
2. **Perder a chave não perde a conta.** Dá para criar outra chave para a mesma inbox.
   A conta nunca fica órfã.
3. **A chave fica guardada** no Gestor de Credenciais do Windows. Se a gravação falhar,
   o AccMaker mostra a chave uma vez e avisa, em vez de dizer que correu tudo bem.

## Apagar não é um pedido

Por baixo, apagar uma conta faz **três pedidos**: apagar as chaves da inbox e só depois
a inbox. Sem isso, a API recusa com `409 Conflict`.

Do teu lado é um botão. Se alguma vez der erro, é porque a API recusou — o AccMaker
mostra o estado HTTP e o código que a API devolveu.

## Limites

| | |
|---|---|
| Contas | **3** |
| Emails por dia | 100 |
| Emails por mês | 3 000 |
| Armazenamento | 3 GB |
| Domínio próprio | não (só `nome@agentmail.to`) |
| Rodapé nas mensagens enviadas | sim, "Sent via AgentMail" |

A partir da 4.ª conta, o AccMaker diz *"Chegaste ao limite de 3 contas"* e recusa.
Não tenta contornar, e não há modo de contornar por configuração.

Se um dia 3 não chegar: o plano Developer do AgentMail tem 10 inboxes por $20/mês, ou
o Purelymail dá mailboxes ilimitadas por $10/ano com password escolhida por ti. As
duas coisas estão atrás da mesma interface interna.

> ⚠️ Cria inboxes num fornecedor de API. Não contorna verificações de serviços
> públicos (Gmail, Outlook, etc.), não usa emails descartáveis e não cria contas
> múltiplas para contornar limites de plano.

## Estado

Funciona hoje em **linha de comandos**. A janela em Windows é o passo seguinte.

```powershell
.\target\release\accmaker.exe ajuda
```

Comandos: `listar`, `criar`, `apagar <id>`, `trocar <id>`, `password`, `testar-chave
<id>`, `criar-chave <id>`.

Todas as operações que gastam uma vaga ou apagam algo **pedem confirmação**, e
`--dry-run` mostra o pedido HTTP sem o enviar.

### A chave da organização

O AccMaker precisa de uma API key **de organização** (não uma `am_us_inbox_`), que fica
na consola do AgentMail:

```powershell
$env:AGENTMAIL_API_KEY = Read-Host 'AgenteMail API key'
```

O `Read-Host` é de propósito: escrever `$env:... = "a-key"` deixaria a chave no
histórico do PowerShell, em texto simples. A chave nunca é lida de um argumento de
linha de comandos, nunca vai para um log e nunca é escrita em ficheiro.

## Requisitos de design

| Requisito | Estado |
|---|---|
| Um único `.exe`, sem DLLs nem pastas | cumprido — copiado sozinho para `%TEMP%` e executado |
| Tamanho | **1,23 MB** (alvo: < 2 MB) |
| Segredos fora do `.exe` | cumprido — Gestor de Credenciais do Windows |
| Arranque instantâneo | cumprido |

## Decisões técnicas que custaram

- **Rust**, com `opt-level="z"`, LTO e `strip`. A interface vai ser Win32 nativo
  porque uma janela com `egui` custava 2,85 MB contra 0,11 MB do Win32.
- **`ureq` + `rustls`** em vez de WinHTTP. O WinHTTP media 0,11 MB mas falhava nesta
  máquina; o `ureq` faz HTTPS e custa 1,06 MB a mais, ainda dentro do alvo.
- **Decidir pelos `code` da API, nunca pelo estado HTTP.** O AgentMail responde `403`
  tanto para "o username já existe" como para "sem permissão".
- **O mock não sabe o protocolo.** Vários bugs só apareceram com a conta real: o `409`
  ao apagar, o `403 already_exists`. Os testes com mock garantem a lógica, não a API.

## Mais informação

- [`docs/pesquisa-api-email.md`](docs/pesquisa-api-email.md) — escolha do fornecedor,
  preços e as 8 perguntas testadas com uma conta real.
- [`progresso.md`](progresso.md) — estado e decisões em aberto.
- [`AGENTS.md`](AGENTS.md) e `.opencode/skills/` — como este repo é trabalhado.

## Licença

Ver [`LICENSE`](LICENSE).