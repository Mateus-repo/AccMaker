# Progresso — AccMaker

> Atualizado pela skill `progresso` no fim de cada tarefa. Mais recente primeiro no registo.

## Estado atual
- Fase: 2 — Gerador de passwords pronto (CLI)
- Linguagem: **Rust 1.99.0** (instalado via rustup nesta máquina)
- Perfis de build: `opt-level="z"`, `lto`, `codegen-units=1`, `panic="abort"`, `strip`
- Fornecedor de API: **Purelymail — API confirmada em fonte primária** (swagger oficial).
  **Migadu riscado** — os termos proíbem "automation of any kind".
- Tamanho atual do .exe: **131 584 B — 0,13 MB** (`accmaker.exe`, gerador de passwords)
- Alvo de tamanho: < 5 MB (ideal < 2 MB)

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
- [ ] **Aguardar resposta do suporte** (limites de password, domínio partilhado)
- [ ] **Medir WinHTTP vs ureq+rustls** com um pedido HTTPS real
- [ ] Interface `provider` com `check_username` / `create_mailbox`
- [ ] Cliente Purelymail (o domínio entra como parâmetro)
- [ ] Interface gráfica (Win32 nativo) e ajuste da skill `estilizacao`
- [ ] Guardar config/API key fora do .exe
- [ ] Build final standalone e medição de tamanho
- [ ] Release no GitHub

## Registo
| Data | Tarefa | Resultado | Commit |
|---|---|---|---|
| 2026-10-12 | Gerador de passwords em CLI, 13 testes a passar | feito | — |
| 2026-10-12 | Swagger oficial do Purelymail confirmado; dominio partilhado | feito | — |
| 2026-10-12 | Medicao de tamanho: CLI / Win32 / egui | feito | b52d368 |
| 2026-10-12 | Regra de escrita PT-PT no AGENTS.md | feito | 7103456 |
| 2026-10-12 | Correção da pesquisa: swagger inexistente + termos verificados | feito | cae0749 |
| 2026-10-12 | Pesquisa da API de email (`docs/pesquisa-api-email.md`) | feito | 6d79550 |
| 2026-10-12 | Estrutura inicial: AGENTS.md, README, skills | feito | 5ba717a |

## Problemas / decisões em aberto
- **Pendente do suporte**: limites de password do `createUser` e se aceita domínio
  partilhado. Pergunta em `docs/pesquisa-api-email.md`, secção 8. Até responder, o
  gerador usa um teto prudente de 64 (`TAMANHO_MAXIMO_PRUDENTE`).
- **Por medir**: tamanho com TLS. WinHTTP (via `windows-sys`) deixa a parte de TLS a
  cargo do Windows; `ureq` + `rustls` traz o stack inteiro e pode acrescentar MB.
  Medir antes de escolher, porque afeta o tamanho final.
- **Decidido**: GUI em Win32 nativo (106 KB contra 2,85 MB do egui). A skill
  `estilizacao` tem de ser ajustada à linguagem dos controlos nativos, ou assume-se
  uma aparência mais simples. A decidir antes da fase de interface.
- **Decidido**: Migadu fora — Acceptable Use proíbe "automation of any kind".
- **Decidido**: `provider` com `check_username` / `create_mailbox`, domínio como
  parâmetro, para trocar de fornecedor sem tocar na UI nem no gerador.
