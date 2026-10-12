# Progresso — AccMaker

> Atualizado pela skill `progresso` no fim de cada tarefa. Mais recente primeiro no registo.

## Estado atual
- Fase: 4 — Cliente AgentMail e CLI prontos (ainda sem pedidos reais)
- Linguagem: **Rust 1.99.0** (instalado via rustup nesta máquina)
- Perfis de build: `opt-level="z"`, `lto`, `codegen-units=1`, `panic="abort"`, `strip`
- Fornecedor de API: **AgentMail, plano grátis** (3 inboxes). Documentação oficial
  confirmada. **Purelymail** fica como opção paga atrás da mesma interface.
  **Migadu riscado** — os termos proíbem "automation of any kind".
- Tamanho atual do .exe: **1 277 440 B — 1,22 MB** (com `ureq` + `rustls`)
- Alvo de tamanho: < 5 MB (ideal < 2 MB) — **cumprido**
- Testes: **35 a passar**
- HTTP: `ureq` + `rustls` (o WinHTTP falhou com erro 12005 nesta máquina)

## API do Purelymail — confirmada (2026-10-12)

Especificação oficial em <https://news.purelymail.com/api/index.html>, com o JSON em
`swagger-spec.js` (36 KB). Título "Purelymail API" v0.0.1, 19 endpoints, todos `POST`
em `/api/v0/`. Autenticação: header `Purelymail-Api-Token` (apiKey), sem OAuth.

`CreateUserRequest`: obrigatórios `userName`, `domainName`, `password`; opcionais
`enablePasswordReset`, `recoveryEmail`, `recoveryPhone`, `enableSearchIndexing`,
`sendWelcomeEmail`.

O campo `password` está declarado sem `minLength`, `maxLength` ou `pattern` — a
especificação **não impõe nem documenta limites**.

**Não é preciso comprar domínio.** Há domínios partilhados (`purelymail.com`) a
$0,02–0,20 por username e por ano. Confirmado em `purelymail.com/docs/users`.

Pendente de resposta do suporte: limites de password, e se o `createUser` aceita
domínio partilhado.

## Medição do HTTPS: WinHTTP vs ureq+rustls (2026-10-12)

Ambas compiladas em release com o mesmo perfil, com um pedido HTTPS real a
`purelymail.com/api/v0/listDomains`.

| Opção | Tamanho | TLS | Pedido real |
|---|---|---|---|
| **WinHTTP** (`windows-sys`) | **111 104 B — 0,11 MB** | a cargo do Windows (schannel) | **falhou** (erro 12005) |
| **ureq + rustls** | **1 226 240 B — 1,17 MB** | dentro do binário | **estado 200** |

O `ureq` traz o stack TLS inteiro: **1,06 MB a mais** que o WinHTTP, o que é
praticamente todo o orçamento de 2 MB. Mesmo assim fica dentro do alvo.

O WinHTTP **não chegou a fazer o pedido**: devolveu erro de sistema 12005 contra
`purelymail.com` e também contra `example.org`, portanto não é do site nem da rede
(o `ureq` fez o mesmo pedido com estado 200). Não consegui isolar a causa — testei
sem deteção de proxy e sem a flag de ignorar CA, e o erro persiste. Pode ser uma
política de segurança desta máquina. **Fica por investigar antes de escolher.**

Os números de tamanho são fiáveis (binários compilados e medidos); a fiabilidade
funcional do WinHTTP **não** está demonstrada.

## Gerador de passwords

`accmaker.exe` com 16 testes a passar, incluindo 10 000 passwords com as quatro
classes ativas, a confirmar que todas aparecem em cada uma.

Bug corrigido nesta tarefa: a zona de rejeição em `indice_sem_enviesamento` estava
calculada sobre o intervalo de 16 bits, mas o RNG devolve 1 byte. Com um byte a zona
correta é `(256 / n) * n`; a anterior aceitava valores que introduzem enviesamento
de módulo. Corrigido e coberto por um teste de distribuição.

## Medições reais do tamanho (2026-10-12)

Compiladas em release com o perfil da skill `build-exe-leve`, Rust 1.99.0.
Cada variante é um "olá mundo" com o mínimo de cada opção.

| Variante | Dependências | Tamanho | Cabe nos 2 MB? |
|---|---|---|---|
| **CLI** (sem GUI) | nenhuma | **110 080 B — 0,11 MB** | sim, com folga |
| **Win32 nativo** (janela + botão) | `windows-sys` | **105 984 B — 0,11 MB** | sim, com folga |
| **egui** (janela + botão) | `egui` + `eframe` | **2 992 640 B — 2,85 MB** | **não** |

Os três foram copiados sozinhos para uma pasta vazia em `%TEMP%` e executaram sem
faltar nada. Nenhum precisa de DLLs ou pastas ao lado.

Leitura: o `egui` **estoura** o alvo ideal de 2 MB só com a janela e um botão, antes
de qualquer logica. O Win32 nativo dá a mesma janela por 106 KB — 28 vezes mais leve.
Como o `egui` fica dentro do alvo de 5 MB mas falha no ideal, e o Win32 custa quase
nada em tamanho, a escolha é **Win32 nativo** quando a GUI entrar.

Nenhum destes números é uma previsão: sao de binarios compilados e medidos.

## Checklist
- [x] Pesquisa e escolha da API de email
- [x] Escolha da linguagem + medição de tamanho (Rust; GUI em Win32 nativo)
- [x] Gerador de passwords em CLI (com testes)
- [x] **API do Purelymail confirmada** em swagger oficial
- [x] **Mudar para AgentMail grátis**; docs, README e AGENTS actualizados
- [x] **Skill `provider-agentmail`** instalada e corrigida com factos verificados
- [x] **Interface `provider`** com fornecedor simulado e testes de slots
- [x] Medição de tamanho com HTTPS (WinHTTP vs ureq+rustls)
- [x] **Cliente AgentMail** (listar, criar, apagar, key restrita) com `--dry-run`
- [x] **CLI** com `listar`, `criar`, `apagar`, `trocar`, `password`
- [x] **Conta órfã tratada**: se a gravação da chave falhar, mostra-a uma vez e avisa
- [ ] **Teste real** com conta gratuita: 3 inboxes, apagar, criar; registar respostas
- [ ] Interface gráfica (Win32 nativo) com os 3 slots
- [ ] Build final standalone e medição de tamanho
- [ ] Release no GitHub

## Registo
| Data | Tarefa | Resultado | Commit |
|---|---|---|---|
| 2026-10-12 | Regra da conta orfa + registo do pedido unico; 38 testes | feito | — |
| 2026-10-12 | Cliente AgentMail + CLI; 35 testes | feito | 72b6de1 |
| 2026-10-12 | Interface provider + slots com mock; 27 testes | feito | 257e4a4 |
| 2026-10-12 | Mudanca para AgentMail: docs, README, AGENTS, skill | feito | 257e4a4 |
| 2026-10-12 | Medicao HTTPS: WinHTTP vs ureq+rustls | parcial | ac8a0b3 |
| 2026-10-12 | Corrigido enviesamento de modulo no gerador; 16 testes | feito | ac8a0b3 |
| 2026-10-12 | Gerador de passwords em CLI, 13 testes a passar | feito | 265064f |
| 2026-10-12 | Swagger oficial do Purelymail confirmado; dominio partilhado | feito | ac8a0b3 |
| 2026-10-12 | Medicao de tamanho: CLI / Win32 / egui | feito | b52d368 |
| 2026-10-12 | Regra de escrita PT-PT no AGENTS.md | feito | 7103456 |
| 2026-10-12 | Correção da pesquisa: swagger inexistente + termos verificados | feito | cae0749 |
| 2026-10-12 | Pesquisa da API de email (`docs/pesquisa-api-email.md`) | feito | 6d79550 |
| 2026-10-12 | Estrutura inicial: AGENTS.md, README, skills | feito | 5ba717a |

## Problemas / decisões em aberto
## Teste real — 2026-10-12 (conta AgentMail Free)

Feito com a app compilada, contra a API a sério.

| # | Pergunta | Resposta |
|---|---|---|
| 1 | Apagar liberta a vaga logo? | **Sim** — mas exige apagar as keys da inbox primeiro |
| 2 | Username apagado pode ser reutilizado? | **Sim** — o `1-4717` foi recriado sem problema |
| 2b | Username já em uso? | **403 `already_exists`** — "Inbox already exists" |
| 3 | Erro da 4.ª inbox | nunca chega à API; o AccMaker recusa pela contagem |
| 4 | Key restrita funciona em IMAP? | por testar (Thunderbird) |
| 5 | A key só é mostrada uma vez? | por testar |
| 6 | A key restrita apaga a própria inbox? | por testar |
| 7 | Dá para criar 2.ª key para a mesma inbox? | por testar (consola web) |

### Descobertas que só o teste real deu

**O `DELETE` devolvia 409 Conflict.** A AgentMail recusa apagar uma inbox que ainda
tenha API keys. Corrigido em `ff54b01`: o `apagar` lista as keys
(`GET /v0/inboxes/{id}/api-keys`), apaga cada uma, e só depois apaga a inbox.
Isto estava a partir o `trocar`, que era o pedido principal.

**Erro falado demais.** O cliente juntava 401 e 403 na mesma mensagem. A API usa o 403
para "Inbox already exists", que não é problema de permissão. Corrigido em `904dfb3`:
o erro passa a mostrar o `code` e a `message` reais da API, e o mapeamento decide pelo
`code` e não pelo estado HTTP.

**O estado HTTP engana.** O `already_exists` vem em 403; o limite de inboxes nunca
chega a ser pedido. Por isso a guarda de capacidade no cliente é o que realmente
protege o limite de 3.

**Um erro ao criar não deixa lixo.** Depois do `create` falhar, a listagem continua com
as mesmas contas. A inbox não fica criada a meio.

### Limite dos testes com mock

Os 40 testes usam um fornecedor simulado que **não sabe nada dos estados HTTP que a
API devolve**. Nenhum deles podia apanhar o 409 nem o 403 `already_exists`. Assumir
que o mock cobre o protocolo é errado; o que ele garante é a lógica de slots e a
segurança das passwords.
- **Decisão de interface dependente da 6**: se a key restrita puder apagar a inbox, a
  app não deve mostrar a key em claro por defeito; só um botão "copiar".
- **Conta órfã — regra implementada**, à espera do teste real para confirmar:
  - sem `--yes` (com humano): mostra a chave uma vez e pergunta "ja guardaste a
    chave?". Se sim, a inbox fica — recebe email. Se não, apaga.
  - com `--yes` (script, ninguém lê o ecrã): **apaga a inbox**, porque mostrar uma
    chave que ninguém vai ler só deixa uma conta inutilizável a ocupar uma vaga.
  - testes: `conta_orfa_quando_a_gravacao_falha`, `orfa_com_yes_apaga_a_inbox`,
    `orfa_apaga_a_inbox_certa`.
- **Pergunta 8 por testar**: a key principal consegue criar outra key restrita para
  uma inbox já existente? Se sim, a conta nunca fica sem chave e o problema da órfã
  desaparece por completo.
- **Pedidos reais feitos até agora**: **nenhum ao AgentMail**. Houve um único pedido
  de rede, na medição do TLS do dia 2026-10-12:
  - `POST https://purelymail.com/api/v0/listDomains`
  - cabecalho `Purelymail-Api-Token: <API_TOKEN>` (placeholder literal, sem valor real)
  - corpo `{"includeShared":true}`
  - devolveu **HTTP 200** com o corpo
    `{"type":"error","code":"invalidToken","message":"Token not valid."}`

  Isto **nao** provou que a API do Purelymail aceita pedidos: o Purelymail responde
  200 com um corpo de erro, porque assim funciona a sua API. O que provou foi apenas
  que o `ureq` + `rustls` faz HTTPS e sabe ler a resposta. Nada foi criado, nada foi
  apagado, nenhuma conta foi tocada.

  **Consequencia para o cliente**: na API do Purelymail o estado HTTP não chega para
  saber se houve erro — e preciso ler o `type` do corpo. O cliente do AgentMail, ao
  contrario, usa estados HTTP normais (401/404/422/429), por isso a verificação por
  estado é suficiente lá.
- **Não criar contas múltiplas** no AgentMail para contornar o limite de 3. Está na
  skill e no `AGENTS.md`.
- **"Permanente" com sentido próprio**: a inbox não expira (confirmado), mas não é um
  email pessoal normal — a app tem de o dizer com clareza.
- **Decidido**: Migadu fora — Acceptable Use proíbe "automation of any kind".
- **Decidido**: GUI em Win32 nativo; a skill `estilizacao` já ajustada ao que dá.
