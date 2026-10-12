# Progresso — AccMaker

> Atualizado pela skill `progresso` no fim de cada tarefa. Mais recente primeiro no registo.

## Estado atual
- Fase: 1 — Escolha da linguagem (a seguir)
- Linguagem: _por decidir_ (ver skill `build-exe-leve`)
- Fornecedor de API: **Purelymail** (pendente de confirmação oficial).
  **Migadu riscado** — os termos proíbem "automation of any kind".
- Tamanho atual do .exe: —
- Alvo de tamanho: < 5 MB (ideal < 2 MB)

## Checklist
- [x] Pesquisa e escolha da API de email
- [ ] **Aguardar resposta do suporte Purelymail** (limites de password + suporte da API)
- [ ] Escolha da linguagem + esqueleto do projeto
- [ ] Gerador de passwords (com testes) — **independente do fornecedor**
- [ ] Interface `provider` com `check_username` / `create_mailbox`
- [ ] Cliente Purelymail (só após resposta do suporte)
- [ ] Interface estilizada
- [ ] Guardar config/API key fora do .exe
- [ ] Build final standalone e medição de tamanho
- [ ] Release no GitHub

## Registo
| Data | Tarefa | Resultado | Commit |
|---|---|---|---|
| 2026-10-12 | Correção da pesquisa: swagger inexistente + termos verificados | feito | — |
| 2026-10-12 | Pesquisa da API de email (`docs/pesquisa-api-email.md`) | feito | 6d79550 |
| 2026-10-12 | Estrutura inicial: AGENTS.md, README, skills | feito | 5ba717a |

## Problemas / decisões em aberto
- **Bloqueante**: limites de password do `createUser` desconhecidos. Pergunta enviada ao
  suporte do Purelymail (secção 8 de `docs/pesquisa-api-email.md`). A resposta define o
  máximo do slider (`provider.max_password_length`). Não bloqueia o gerador nem a UI.
- **Bloqueante**: API do Purelymail não tem documentação oficial (404 em `/docs` e no
  swagger). Não escrever o cliente antes de confirmação oficial.
- **Decidido**: Migadu fora — Acceptable Use proíbe "automation of any kind" e permite
  terminação sem aviso. API bem documentada mas incompatível com o caso de uso.
- **Decidido**: arquitetura com `provider` atrás de `check_username` / `create_mailbox`,
  para trocar de fornecedor sem tocar na UI nem no gerador.
- Ordem de precedência do gerador de passwords: catálogo de caracteres a definir.
