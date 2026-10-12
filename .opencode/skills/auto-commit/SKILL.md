---
name: auto-commit
description: Faz commit automático e seguro no fim de cada tarefa concluída do AccMaker, com mensagem convencional em PT-PT, sem segredos, e push opcional se existir remote. Usar depois da skill progresso.
---

# Auto commit

## Quando
No fim de cada tarefa concluída, **depois** de atualizar `progresso.md`.

## Passos
1. `git status --short` — se não houver alterações, parar.
2. **Verificar segredos** nas alterações: procurar API keys, tokens, passwords, `.env`.
   Se encontrar algo, **não** commitar: remover/ignorar e avisar o utilizador.
3. Garantir que `.gitignore` cobre `*.exe`, `target/`, `dist/`, `.env`, `*.key`.
4. `git add -A` (ou apenas os ficheiros da tarefa).
5. Mensagem no formato `tipo(âmbito): descrição curta no imperativo` (≤ 72 caracteres), por exemplo:
   - `feat(passwords): gerar password com RNG criptográfico`
   - `docs(pesquisa): comparar APIs de email`
   - `chore(build): ativar perfil release leve`
   Tipos: `feat`, `fix`, `refactor`, `docs`, `chore`, `style`, `test`.
6. `git commit -m "<mensagem>"`.
7. Se existir remote (`git remote -v` não vazio): `git push` na branch atual.
   Se falhar por autenticação ou conflito, **não forçar** — informar o utilizador.
8. Reportar o hash do commit (`git rev-parse --short HEAD`).

## Proibido
- `git push --force`, `git reset --hard`, reescrever histórico.
- Commitar binários (`.exe`), ficheiros de configuração local ou qualquer segredo.
- Misturar tarefas diferentes no mesmo commit.
