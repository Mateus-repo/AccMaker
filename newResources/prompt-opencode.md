# Prompt para colar no opencode

Muda de planos: o fornecedor passa a ser o **AgentMail no plano gratis (3 inboxes)**, porque o
requisito agora e **gratis**. O Purelymail fica como opcao paga futura, atras da mesma interface `provider`.

Le primeiro a skill `provider-agentmail` (copiei-a para `.opencode/skills/provider-agentmail/SKILL.md`).

Tarefas, uma de cada vez, com `progresso` e `auto-commit` no fim de cada:
1. Atualizar `docs/pesquisa-api-email.md`: fornecedor escolhido = AgentMail (gratis, 3 inboxes), com os factos da skill e a lista "a verificar". Atualizar README e AGENTS.md (a ferramenta ja nao exige dominio proprio).
2. Acrescentar a tabela de skills do AGENTS.md a linha `provider-agentmail`.
3. Definir a interface `provider` (list, create, delete, capacity, supports_custom_password) e um fornecedor simulado (mock) com testes: criar ate 3, falhar na 4.a, apagar e criar outra.
4. Medir WinHTTP vs ureq+rustls com um pedido HTTPS real e registar os tamanhos.
5. Implementar o cliente AgentMail (list, create com `client_id`, delete, key restrita a uma inbox) com `--dry-run`.
6. CLI com os comandos `listar`, `criar`, `apagar <id>` e `trocar <id>`.
7. So depois, a janela Win32 com os 3 slots e os botoes Apagar, Criar e Trocar.

Regras: nao criar varias contas no AgentMail para contornar o limite, nunca registar keys, escrever so em PT-PT, e reler os ficheiros antes do commit.
