# Progresso — AccMaker

> Atualizado pela skill `progresso` no fim de cada tarefa. Mais recente primeiro no registo.

## Estado atual
- Fase: 1 — Linguagem escolhida: **Rust**. GUI adiada.
- Linguagem: **Rust 1.99.0** (instalado via rustup nesta máquina)
- Perfis de build: `opt-level="z"`, `lto`, `codegen-units=1`, `panic="abort"`, `strip`
- Fornecedor de API: **Purelymail** (pendente de confirmação oficial).
  **Migadu riscado** — os termos proíbem "automation of any kind".
- Tamanho atual do .exe: **110 KB** (CLI de medição, sem interface)
- Alvo de tamanho: < 5 MB (ideal < 2 MB)

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
- [ ] **Aguardar resposta do suporte Purelymail** (limites de password + suporte da API)
- [ ] Gerador de passwords em CLI (com testes) — **independente do fornecedor**
- [ ] Interface `provider` com `check_username` / `create_mailbox`
- [ ] Cliente Purelymail (só após resposta do suporte)
- [ ] Interface estilizada
- [ ] Guardar config/API key fora do .exe
- [ ] Build final standalone e medição de tamanho
- [ ] Release no GitHub

## Registo
| Data | Tarefa | Resultado | Commit |
|---|---|---|---|
| 2026-10-12 | Medicao de tamanho: CLI / Win32 / egui | feito | — |
| 2026-10-12 | Regra de escrita PT-PT no AGENTS.md | feito | 7103456 |
| 2026-10-12 | Correção da pesquisa: swagger inexistente + termos verificados | feito | cae0749 |
| 2026-10-12 | Pesquisa da API de email (`docs/pesquisa-api-email.md`) | feito | 6d79550 |
| 2026-10-12 | Estrutura inicial: AGENTS.md, README, skills | feito | 5ba717a |

## Problemas / decisões em aberto
- **Bloqueante**: limites de password do `createUser` desconhecidos. Pergunta **redigida
  e por enviar** ao suporte do Purelymail (secção 8 de `docs/pesquisa-api-email.md`).
  A resposta define o máximo do slider (`provider.max_password_length`). Não bloqueia o
  gerador nem a interface.
- **Bloqueante**: API do Purelymail não tem documentação oficial (404 em `/docs` e no
  swagger). Não escrever o cliente antes de confirmação oficial.
- **Decidido**: Migadu fora — Acceptable Use proíbe "automation of any kind" e permite
  terminação sem aviso. API bem documentada mas incompatível com o caso de uso.
- **Decidido**: arquitetura com `provider` atrás de `check_username` / `create_mailbox`,
  para trocar de fornecedor sem tocar na UI nem no gerador.
- Ordem de precedência do gerador de passwords: catálogo de caracteres a definir.
