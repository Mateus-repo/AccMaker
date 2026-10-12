---
name: progresso
description: Atualiza o ficheiro progresso.md (estado atual, checklist, registo e decisões em aberto) no fim de cada tarefa do AccMaker. Usar sempre antes do auto-commit.
---

# Manter o progresso.md

## Quando
No fim de **cada tarefa concluída** (ou bloqueada), **antes** da skill `auto-commit`.

## Passos
1. Abrir `progresso.md` (se não existir, criar a partir do modelo abaixo).
2. Atualizar **Estado atual**: fase, linguagem, fornecedor, tamanho atual do .exe (medido, não estimado).
3. Marcar `[x]` os itens da **Checklist** concluídos; acrescentar novos itens se surgirem.
4. Acrescentar uma linha **no topo** da tabela de **Registo**: data (AAAA-MM-DD), tarefa, resultado (`feito` / `parcial` / `bloqueado`) e hash do commit anterior ou `—`.
5. Atualizar **Problemas / decisões em aberto** (remover resolvidos, adicionar novos).
6. Manter o ficheiro curto (máx. ~120 linhas): resumir registos com mais de 30 entradas.

## Modelo
```markdown
# Progresso — AccMaker
## Estado atual
- Fase: …
- Linguagem: …
- Fornecedor de API: …
- Tamanho atual do .exe: …
## Checklist
- [ ] …
## Registo
| Data | Tarefa | Resultado | Commit |
|---|---|---|---|
## Problemas / decisões em aberto
- …
```

## Regras
- Escrever em PT-PT, factual, sem inventar resultados.
- Nunca incluir passwords, API keys ou emails reais criados nos testes.
