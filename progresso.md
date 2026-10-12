# Progresso — AccMaker

> Atualizado pela skill `progresso` no fim de cada tarefa. Mais recente primeiro no registo.

## Estado atual
- Fase: 0 — Planeamento (concluída); a seguir: escolher linguagem
- Linguagem: _por decidir_ (ver skill `build-exe-leve`)
- Fornecedor de API: **Purelymail** (recomendado), alternativa Migadu
- Tamanho atual do .exe: —
- Alvo de tamanho: < 5 MB (ideal < 2 MB)

## Checklist
- [x] Pesquisa e escolha da API de email
- [ ] Escolha da linguagem + esqueleto do projeto
- [ ] Gerador de passwords (com testes)
- [ ] Cliente da API + `--dry-run`
- [ ] Interface estilizada
- [ ] Guardar config/API key fora do .exe
- [ ] Build final standalone e medição de tamanho
- [ ] Release no GitHub

## Registo
| Data | Tarefa | Resultado | Commit |
|---|---|---|---|
| 2026-10-12 | Pesquisa da API de email (`docs/pesquisa-api-email.md`) | feito | — |
| 2026-10-12 | Estrutura inicial: AGENTS.md, README, skills | feito | — |

## Problemas / decisões em aberto
- Mínimo/máximo de password aceite pelo Purelymail não está documentado — testar
  empiricamente antes de fixar o slider de tamanho.
- A API do Purelymail está documentada apenas por ferramentas de comunidade (Raycast,
  MCP), não pelo site de documentação; validar um `createUser` real antes de depender
  dela.
- Ordem de precedence do Gerador de passwords: catálogo de caracteres a definir.
