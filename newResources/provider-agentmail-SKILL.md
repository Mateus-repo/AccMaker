---
name: provider-agentmail
description: Implementar e manter o fornecedor AgentMail (plano gratis, 3 inboxes) no AccMaker, com listar, criar e apagar inboxes a qualquer momento para trocar uma conta por outra. Usar ao escrever o modulo provider, a gestao de slots ou a interface de contas.
---

# Fornecedor AgentMail (plano gratis)

## Objetivo
O utilizador tem no maximo **3 inboxes gratis**. A app tem de permitir, a qualquer momento,
**apagar uma inbox e criar outra** para ocupar o slot libertado, de forma rapida e segura.

## Factos confirmados na documentacao oficial (docs.agentmail.to)
- Autenticacao: `Authorization: Bearer <API key>`.
- Listar: `GET /v0/inboxes`. Obter: `GET /v0/inboxes/:inbox_id`.
- Criar: `POST /v0/inboxes` com campos opcionais `username`, `domain`, `display_name`, `client_id`, `metadata`.
  `client_id` serve para repetir o pedido sem criar duplicados.
- Apagar: `DELETE /v0/inboxes/:inbox_id` (404 se nao existir).
- Plano Free: 3 inboxes, 3 000 emails por mes, 100 por dia, 3 GB; sem dominios proprios.
- IMAP `imap.agentmail.to:993` e SMTP `smtp.agentmail.to` (465 ou 587): utilizador = email da inbox,
  **password = uma API key**.
- Existem **API keys restritas a uma inbox** (`inboxes.api_keys.create(inbox_id, name)`).

## Consequencia importante: a password
O AgentMail **nao tem password escolhida pelo utilizador**. A "password" de cada conta e uma API key.
Para ter "email + password" por conta:
1. criar a inbox;
2. criar uma API key **restrita a essa inbox**;
3. mostrar `email` + essa key como password (e so mostrar uma vez, se a API assim o obrigar).

Por isso, neste fornecedor:
- o gerador de passwords e o slider de tamanho **nao se aplicam** (`supports_custom_password = false`);
- esconder o slider na interface quando o fornecedor ativo for o AgentMail;
- o gerador continua valido para outros fornecedores (ex.: Purelymail).

## Interface do modulo (trait/interface comum a todos os fornecedores)
- `list_inboxes() -> Vec<Inbox>` (id, email, data de criacao)
- `create_inbox(username: Option<String>) -> Credentials` (email + password/key)
- `delete_inbox(inbox_id)`
- `capacity() -> Capacity { used, max }` (max = 3 no plano gratis, configuravel)
- `supports_custom_password() -> bool`
- Modo `--dry-run`: simula tudo sem chamar a API.

## Fluxo de trocar uma conta (o pedido principal)
1. Mostrar os slots: ate 3 contas ocupadas e as vagas livres, vindo de `list_inboxes()` (fonte da verdade e a API, nao o ficheiro local).
2. **Apagar**: pedir confirmacao clara ("apaga o email e todas as mensagens; nao da para anular"). Depois `delete_inbox` e voltar a listar.
3. **Criar**: so ativo se `used < max`. Usar `client_id` aleatorio por tentativa de criacao para repetir sem duplicar.
4. Atalho **Trocar**: apagar + criar numa so acao, mas so criar se o apagar tiver sucesso e a listagem seguinte confirmar a vaga.
5. Mostrar o resultado com botoes de copiar para o email e para a password/key.

## Erros a tratar com mensagens humanas
- 401/403: API key invalida ou sem permissoes.
- 404 ao apagar: ja nao existe; atualizar a lista.
- 429: esperar o `Retry-After`.
- Limite de inboxes atingido: dizer "apaga uma conta primeiro", nunca tentar contornar.
- Rede em baixo: nunca assumir que apagou; voltar a listar quando houver ligacao.

## Armazenamento
- A API key principal fica no Credential Manager do Windows (ou DPAPI), nunca em ficheiro nem no git.
- Ficheiro local em `%APPDATA%\AccMaker\` so com ids e emails (nada secreto).
- Nunca registar keys em logs nem em mensagens de erro.

## O que NAO fazer
- **Nao** criar varias contas/organizacoes no AgentMail para contornar o limite de 3 inboxes.
- Nao gerar emails descartaveis nem contornar limites ou termos do fornecedor.

## A verificar com um teste real (nao confirmado na documentacao)
1. Apagar uma inbox liberta logo a vaga no plano gratis?
2. O username de uma inbox apagada pode ser reutilizado?
3. Qual e o erro exato ao criar a 4.a inbox?
4. A key restrita a uma inbox funciona em IMAP no plano gratis?
5. A key restrita so e mostrada uma vez?
6. Existe expiracao de mensagens no plano gratis (a pagina de precos menciona "message expiry")?
Registar as respostas em `docs/pesquisa-api-email.md` com a data do teste.

## Testes
- Testes unitarios com um fornecedor simulado (mock): listar, criar ate ao limite, falhar na 4.a, apagar e criar de novo.
- Teste com `--dry-run` sem rede.
- Depois do teste real, apagar tudo o que foi criado.
