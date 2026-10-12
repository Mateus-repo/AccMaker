---
name: provider-agentmail
description: Implementar e manter o fornecedor AgentMail (plano gratis, 3 inboxes) no AccMaker, com listar, criar e apagar inboxes a qualquer momento para trocar uma conta por outra. Usar ao escrever o modulo provider, a gestao de slots ou a interface de contas.
---

# Fornecedor AgentMail (plano gratis)

## Objetivo
O utilizador tem no maximo **3 inboxes gratis**. A app tem de permitir, a qualquer momento,
**apagar uma inbox e criar outra** para ocupar o slot libertado, de forma rapida e segura.

## Aviso: o que este fornecedor NAO e

Isto importa para a honestidade do produto e precisa de aparecer na interface:

- **Nao ha password.** A "password" de cada conta e uma **API key**. Se a key for
  apagada ou expirar, a conta fica inutilizavel — nao ha forma de a recuperar.
- **Nao e um email pessoal normal.** O endereco e `nome@agentmail.to` (dominio partilhado
  do AgentMail), sem dominio proprio no plano gratis.
- **Mensagens enviadas levam o rodape "Sent via AgentMail"**, que fica no plano gratis.
- **100 emails por dia**, 3000 por mes, 3 GB. Nao e para uso intensivo.
- **Nao e ilimitado.** O preco de um limite de 3 inboxes e a propria API.

O `AGENTS.md` define o objetivo original como "mailboxes reais e permanentes". Esta
app cria inboxes de um fornecedor de API para agentes, que e outra coisa. Manter a
promessa de "permanente" significa dizer com exactao: **a inbox nao expira**, desde que
a conta nao seja apagada e a key se mantenha valida. Dizer mais do que isso seria
mentir ao utilizador.

## Factos confirmados na documentacao oficial (docs.agentmail.to)

Verificados em 2026-10-12 contra <https://docs.agentmail.to>:
- Autenticacao: `Authorization: Bearer <API key>`.
- Listar: `GET /v0/inboxes`. Obter: `GET /v0/inboxes/:inbox_id`.
- Criar: `POST /v0/inboxes` com campos opcionais `username`, `domain`, `display_name`,
  `client_id`, `status`, `metadata`. `domain` tem de ser um dominio verificado; sem ele,
  assume `agentmail.to`. `client_id` serve para repetir o pedido sem criar duplicados.
  A resposta traz `inbox_id`, `email`, `created_at`, `updated_at`, `pod_id`.
- Apagar: `DELETE /v0/inboxes/:inbox_id` (404 se nao existir).
- API key restrita a uma inbox: `POST /v0/inboxes/{inbox_id}/api-keys`, com corpo
  `{"name": "...", "expires_at": ..., "permissions": {...}}`. A resposta traz
  **`api_key` (o valor completo), `api_key_id`, `prefix` e `inbox_id`**.
  `expires_at` omitido herda a validade da key que a criou.
- Plano Free: 3 inboxes, 3000 emails por mes, 100 por dia, 3 GB; sem dominios proprios;
  sem remocao do rodape; sem white-label; sem cloud na UE.
- IMAP `imap.agentmail.to:993` e SMTP `smtp.agentmail.to` (465 ou 587): utilizador = email
  da inbox, **password = uma API key**.
- Especificacao completa em <https://docs.agentmail.to/openapi.json> e
  <https://docs.agentmail.to/llms.txt>.

### Retencao: confirmado, e desligado

<https://docs.agentmail.to/message-expiry>: a expiracao de mensagens ao fim de 24 horas
e uma **politica opcional, por organizacao, desligada por omissao**, que so se ativa
contactando a equipa. Nao se ativa sozinha. A documentacao de produto descreve as
inboxes do plano gratis como "3 persistent inboxes".

Isto responde a pergunta 6 da lista de verificacao: **nao ha expiracao no plano gratis**.

## Consequencia importante: a password

O AgentMail **nao tem password escolhida pelo utilizador**. A "password" de cada conta e
uma API key. Para ter "email + password" por conta:

1. criar a inbox;
2. criar uma API key **restrita a essa inbox** (`POST /v0/inboxes/{id}/api-keys`);
3. mostrar `email` + essa key como password.

Ponto critico de honestidade: a resposta traz o valor da key em `api_key`, e a
documentacao **nao diz se so aparece uma vez**. Se so aparecer uma vez, e porque a
inbox nao tem outro mecanismo de recuperacao, o valor tem de ser guardado pelo
utilizador **no momento** em que aparece. Se a app o voltar a mostrar mais tarde sem o
ter guardado, e porque o tirou de um armazenamento local — e esse armazenamento passa
a ser um segredo que protege.

Decisao de produto Implementada: guardar a key restrita no **Credential Manager do
Windows**, por inbox, e mostrar sempre a partir dai.

### Se a gravacao da chave falhar

O `CredWriteW` pode falhar (disco cheio, perfil de Windows bloqueado, antivírus). A
inbox ja foi criada nessa altura, e fica **orfa**: existe, recebe email, mas ninguem
consegue entrar porque a key nao ficou em lado nenhum.

Regra implementada no AccMaker, que depende de haver alguem a ver o ecra:

- **Sem `--yes`** (modo normal, com humano): mostrar a key uma unica vez e perguntar
  *"ja guardaste a chave?"*. Se disser que sim, a inbox fica — ate porque ela recebe
  email, e isso tem valor. Se disser que nao, apagar a inbox.
- **Com `--yes`** (script, ninguem ve o ecra): **apagar a inbox**. Mostrar uma key
  que ninguem vai ler so serve para deixar uma conta inutilizavel ocupando uma das 3
  vagas.

Por omissao, apagar e a opcao destrutiva, por isso so acontece sem `--yes` e com a
resposta do utilizador.

**Isto pode mudar depois do teste real**: se a key principal conseguir criar outra key
restrita para uma inbox existente, a conta nunca fica perdida e o problema desaparece.
Ver pergunta 7.

Por isso, neste fornecedor:
- o gerador de passwords e o slider de tamanho **nao se aplicam**
  (`supports_custom_password() == false`);
- **esconder** o slider e os interruptores de caracteres na interface quando o
  fornecedor ativo for o AgentMail, em vez de os mostrar desactivados;
- explicar porquê, numa linha: "neste fornecedor a password e uma API key, nao uma
  password escolhida por ti";
- o gerador continua valido e com testes para outros fornecedores (ex.: Purelymail).

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
- **Nao** criar varias contas/organizacoes no AgentMail para contornar o limite de 3.
- **Nao** usar o registo sem API key para criar contas multiplas e contornar o limite.
  E contra o espirito do plano gratis e arrisca perder todas as inboxes de uma vez.
- Nao gerar emails descartaveis nem contornar limites ou termos do fornecedor.
- Nao mostrar a API key em logs, capturas de ecra ou mensagens de erro.

## A verificar com um teste real (nao confirmado na documentacao)

Teste: criar 3 inboxes, apagar uma, criar outra, e registar o que aconteceu.

1. Apagar uma inbox liberta logo a vaga no plano gratis?
2. O username de uma inbox apagada pode ser reutilizado?
3. Qual e o erro exato ao criar a 4.a inbox (codigo HTTP e `code` do corpo)?
4. A key restrita a uma inbox funciona em IMAP no plano gratis?
5. A key restrita so e mostrada uma vez? Como se recupera se se perder?
6. **Ja respondido**: existe expiracao de mensagens? **Nao.** Politica opcional por
   organizacao, desligada por omissao. Ver `docs/pesquisa-api-email.md`.
7. A key restrita pode apagar a propria inbox? Se sim, um vazamento da key permite
   destruir a conta — avaliar se vale a pena mostrar a key.
8. **A key principal consegue criar outra key restrita para uma inbox ja existente?**
   Se sim, uma conta nunca fica sem chave (basta recriar) e o problema da conta orfa
   desaparece por completo. E o mais importante dos oito, porque decide se vale a pena
   investir na recuperacao.

Registar as respostas em `docs/pesquisa-api-email.md` com a data do teste.

## Testes
- Testes unitarios com um fornecedor simulado (mock): listar, criar ate ao limite, falhar na 4.a, apagar e criar de novo.
- Teste com `--dry-run` sem rede.
- Depois do teste real, apagar tudo o que foi criado.
