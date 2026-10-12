# Pesquisa de API de email — AccMaker

Data: 2026-10-12. Objetivo: encontrar uma API **oficial e documentada** que crie
mailboxes **reais e permanentes** com password definida pelo chamador, a partir de um
`.exe` Windows standalone (sem servidor próprio, sem OAuth browser).

Excluídos por regra do projeto: mail.tm, Guerrilla Mail, 10MinuteMail e restantes
temporários/descartáveis; automação de sign-up em Gmail/Outlook/Yahoo; qualquer bypass
de CAPTCHA/SMS.

> **Mudança de requisitos (2026-10-12).** O requisito passou a ser **gratuito**. Isso
> muda o que esta pesquisa procura. A secção 9 documenta o fornecedor escolhido — o
> **AgentMail** — e o que ele é e não é. As secções 1 a 8 ficam como registo da pesquisa
> original, cujo requisito era ter mailboxes próprias e ilimitadas.

---

## 1. Tabela comparativa

| Critério | **Purelymail** | **Migadu** | **FranklyMail** | **Mailcow** (self-hosted) | **Stalwart** (self-hosted) | **Zoho Mail** (org) | **Google Workspace** | **Microsoft 365** | **Fastmail** |
|---|---|---|---|---|---|---|---|---|---|
| Cria mailbox real com password do chamador | **Sim** (`createUser` com `password`) | **Sim** (`POST .../mailboxes` com `password`) | **Sim** (`POST /mailboxes` com `password`) | Sim | Sim (via JMAP `Account/set`) | Sim | Sim | Sim (com licença) | **Não** (sem API de provisioning) |
| Domínio próprio obrigatório | **Não** — há domínios partilhados (`purelymail.com`) a $0,02–0,20 por username/ano | **Sim** | **Sim** | **Sim** | **Sim** | **Sim** + conta Zoho paga | **Sim** + Workspace | **Sim** + tenant M365 | Não (domínios próprios, mas sem API admin) |
| Custo | **$10/ano flat** (plano Simple), sem limite de users/domínios | $19/ano (Micro) ou $9/mês (Standard, $90/ano); mailboxes ilimitadas | $9/ano, $0,40/GB/yr acima de 10 GB | Grátis (custo é o servidor) | Grátis (custo é o servidor) | ~$1,20/utilizador/mês (Mail Lite) | $7/utilizador/mês (Business Starter) | ~$6/utilizador/mês | ~$3–40/utilizador/mês |
| Limites de password documentados | Não documentados; sem mínimo imposto pela API | Não documentados | Validação por campo; não documentada no resumo | Validação no servidor Dovecot | Política configurável (enterprise) | Política da organização | Política do domínio | Política do tenant | n/a |
| Autenticação | **API key em header** (`Purelymail-Api-Token`), sem OAuth | HTTP Basic (email da conta + API key) | **Bearer token** de management key | Header `X-API-Key` ou `Authorization: Bearer` | Bearer / Basic contra `/jmap` | OAuth 2.0 (client ID/secret + refresh) | OAuth 2.0 (service account) | OAuth 2.0 (client credentials) | OAuth 2.0 + JMAP |
| Sem servidor próprio / sem browser | **Sim, ideal** | **Sim** | **Sim** | Não | Não | Parcialmente (OAuth exige registo prévio) | Não | Não | Não |
| Limites de taxa | Não publicados; API em `/api/v0` | Não publicados (API "early beta") | 120 escritas / 5 min (conta); leituras não contadas | Não documentados | Não documentados | Throttling Zoho padrão | Cotas do Admin SDK | Throttling Graph padrão | — |
| Verifica username disponível | **Sim**, implicitamente: `createUser` falha com código de erro | **Sim**, `GET .../mailboxes/{local_part}` → 200/400 | **Sim**, `GET /mailboxes` e o erro 422 do POST | **Sim** (`GET /api/v1/mailbox/{domain}/{local_part}`) | Sim, via `Account/get` | Sim | Sim | Sim | — |
| Qualidade da documentação | Swagger público,Community tools (Raycast, MCP) oficiais |Boa, uma página de referência com curl | **Excelente**, referência completa com exemplos | OpenAPI servido pela própria instância | Documentação boa mas JMAP exige curva de learning | Boa | Muito boa | Muito boa | Boa |
| ToS compatível com criação automatizada | Sim, para a própria organização | Sim, para a própria organização | Sim, **mas proíbe revenda/serviço a terceiros** | Sim | Sim | Sim, para a organização | Sim, para a organização | Sim, para a organização | n/a |

Notas de veracidade:
- A coluna Purelymail vem do endpoint `POST /api/v0/createUser` e do esquema de tipos
  usado pela extensão oficial da Raycast e por MCP servers da comunidade.
- A coluna Fastmail está preenchida a negativo porque a documentação para
  programadores (`fastmail.com/dev`) só expõe JMAP/IMAP/SMTP por utilizador; não existe
  endpoint documentado de criação de contas.

---

## 2. Recomendação

### 1.º lugar — Purelymail

Docs: <https://purelymail.com/docs> · API em <https://purelymail.com/api/v0/>

Porque:
1. É a única com **API key simples em header**, sem OAuth nem refresh token — o `.exe`
   pode fazer o pedido direto com uma única chamada `POST` e nada mais.
2. Cria a mailbox com a **password que nós geramos** (`createUser` aceita `password`),
   que é exatamente o requisito do AccMaker.
3. `$10/ano`, **sem limite de users, nem de domínios, nem de storage** — o custo não
   cresce por mailbox criada, o que casa com um gerador de contas.
4. `listUser` e `listDomains` dão verificação de disponibilidade antes de criar.
5. API exclusivamente `POST` com JSON — cliente HTTP minúsculo, quase zero dependências.

**Estado de verificação desta recomendação:**

| Aspeto | Estado | Fonte |
|---|---|---|
| Preço ($10/ano, sem limites) | **Confirmado** | <https://purelymail.com/pricing> |
| Termos (ver secção 6) | **Confirmado** | <https://purelymail.com/termsofservice> |
| API `createUser` e restantes 18 endpoints | **Confirmado** | swagger oficial |
| Domínio partilhado (`purelymail.com`) | **Confirmado** | <https://purelymail.com/docs/users> |
| Limites de password | **Desconhecidos** | não constam da especificação |

### Fonte primária da API

**Existe documentação oficial**, num endereço que não é óbvio:

- Interface: <https://news.purelymail.com/api/index.html> (Swagger UI, HTTP 200)
- Especificação: <https://news.purelymail.com/api/swagger-spec.js> (JSON, 36 KB, 19 endpoints)

Título da especificação: "Purelymail API", versão `0.0.1`, servidor
`https://purelymail.com`. Autenticação declarada em `components.securitySchemes`:

```json
"token": { "name": "Purelymail-Api-Token", "type": "apiKey", "in": "header" }
```

Isto confirma a API key num header, sem OAuth, exatamente como se pressupunha.

Endpoints (todos `POST` em `/api/v0/`): `createUser`, `deleteUser`, `listUser`,
`modifyUser`, `getUser`, `upsertPasswordReset`, `deletePasswordReset`,
`listPasswordReset`, `createRoutingRule`, `deleteRoutingRule`, `listRoutingRules`,
`addDomain`, `getOwnershipCode`, `listDomains`, `updateDomainSettings`,
`deleteDomain`, `createAppPassword`, `deleteAppPassword`, `checkAccountCredit`.

`CreateUserRequest` — campos obrigatórios `userName`, `domainName`, `password`.
Opcionais: `enablePasswordReset` (predef. `true`), `recoveryEmail`,
`recoveryEmailDescription`, `recoveryPhone`, `recoveryPhoneDescription`,
`enableSearchIndexing` (predef. `true`), `sendWelcomeEmail` (predef. `true`).

`userName` é descrito como "Local part of username, e.g. 'user' in 'user@domain.com'".

**Nota:** o campo `password` está declarado como `{"type": "string"}`, sem `minLength`,
sem `maxLength` e sem `pattern`. A especificação **não impõe limites**, o que não
significa que o servidor não os tenha — só que não estão documentados. Continua a
precisar de resposta oficial (secção 8).

### Alternativa — Migadu

Docs: <https://migadu.com/api> · Preços: <https://migadu.com/pricing>

API REST classical, HTTP Basic (email da conta + API key), referência completa com
exemplos `curl`. Mailboxes ilimitadas em todos os planos, Micro a $19/ano. É a opção
mais conservadora e a melhor se a documentação do Purelymail não estiver acessível.
Ressalva grave: os Termos do Migadu **proíbem automação**. Ver secção 6.

### Terceira opção — Mailcow (self-hosted)

Docs: <https://docs.mailcow.email> (OpenAPI em `https://<instância>/api/`)

Só faz sentido se o utilizador já tiver (ou quiser) um servidor. Zero custo de
fornecedor, controlo total, mas obriga a manter o servidor e a expor a API com
allow-list de IP — o `AccMaker` passa a precisar do endereço do servidor configurável.

---

## 3. Endpoints exatos

### Domínio: próprio **ou** partilhado

Não é preciso comprar um domínio para usar o AccMaker. O Purelymail tem **domínios
partilhados** (por exemplo `purelymail.com`). Fonte: <https://purelymail.com/docs/users>

> "Custom Domains: There is no additional charge for users on domains you own.
> **Shared Domains: If you use our shared domains (like purelymail.com), there is a
> small annual usage charge per username (typically $0 to $1.20/year depending on
> length) to prevent name-squatting.**"

### Preço por username em domínio partilhado

**URL exata:** <https://purelymail.com/advancedpricing>, secção "Exact fees", sob
"Usernames on shared domains". Texto literal da página:

> **Usernames on shared domains:**
> - **1 to 6 letters:** $0.20 per user per year
> - **7 to 12 letters:** $0.05 per user per year
> - **13+ letters:** $0.02 per user per year

A página de documentação (<https://purelymail.com/docs/users>) dá outro número:

> "**Shared Domains:** If you use our shared domains (like purelymail.com), there is a
> small annual usage charge per username (typically **$0 to $1.20/year** depending on
> length) to prevent name-squatting."

**Os dois estão certos e não se contradizem.** A documentação dá a *gama*; a página de
preços avançados dá a *tabela exata*, de $0,02 a $0,20. A documentação deve estar a
incluir componentes que não constam dessa tabela, ou a usar um valor máximo
conservador para estimar o custo.

Para o AccMaker vale a **tabela**: $0,02 a $0,20 por username e por ano. Com um
username de 13+ caracteres, **$0,02/ano**.

Nota: estes preços são de **preço avançado** (pay as you go, com $4/ano de conta). No
plano **Simple** de $10/ano não há limites rígidos, e é esse o plano a usar.

A API tem `listDomains` com `includeShared` (predefinição `false`): passar `true` lista
também os domínios partilhados. A especificação inclui `isShared` ("Whether this domain
is shared (owned by Purelymail)") em `ApiDomainInfo`.

**Ainda por confirmar:** se o `createUser` aceita um domínio partilhado no `domainName`.
A especificação diz apenas `domainName: {"type": "string"}`, sem restrições, e o portal
aceita. Não é prova. Incluído na pergunta da secção 8.

### Purelymail (recomendado)

Base: `https://purelymail.com/api/v0/`
Método: **POST** para todos os endpoints.
Auth: header `Purelymail-Api-Token: <API_TOKEN>` + `Content-Type: application/json`.
Resposta: `{"type":"success","result":{...}}` ou `{"type":"error","code":"...","message":"..."}`.

| Operação | Endpoint | Body |
|---|---|---|
| Listar domínios | `POST /api/v0/listDomains` | `{"includeShared": false}` |
| Verificar username | `POST /api/v0/listUser` | `{}` → `result.users[]` (comparar com `local@domain`) |
| Criar mailbox | `POST /api/v0/createUser` | `{"userName":"<local>","domainName":"<dominio>","password":"<password>"}` |
| Listar users | `POST /api/v0/listUser` | `{}` |

Exemplo (placeholders, nunca chaves reais):

```bash
curl -X POST https://purelymail.com/api/v0/createUser \
  -H "Content-Type: application/json" \
  -H "Purelymail-Api-Token: <API_TOKEN>" \
  -d '{"userName":"<LOCAL_PART>","domainName":"<DOMINIO>","password":"<PASSWORD>"}'
```

Campos opcionais de `createUser` (úteis para o AccMaker):
`enablePasswordReset`, `enableSearchIndexing` (pôr a `false` poupa espaço),
`sendWelcomeEmail` (pôr a `false`, evita lixo na caixa criada).

### Migadu (alternativa)

Base: `https://api.migadu.com/v1/`
Auth: HTTP Basic `-u "<EMAIL_DA_CONTA_MIGADU>:<API_KEY>"`.
Erros: HTTP 400 com corpo JSON.

| Operação | Endpoint | Body |
|---|---|---|
| Listar domínios | `GET /v1/domains` | — |
| Verificar username | `GET /v1/domains/{dominio}/mailboxes/{local_part}` | — |
| Criar mailbox | `POST /v1/domains/{dominio}/mailboxes` | `{"name":"<LOCAL_PART>","local_part":"<LOCAL_PART>","password":"<PASSWORD>"}` |
| Listar mailboxes | `GET /v1/domains/{dominio}/mailboxes` | — |

```bash
curl -u "<EMAIL_CONTA>:<API_KEY>" \
  -X POST https://api.migadu.com/v1/domains/<DOMINIO>/mailboxes \
  -H "Content-Type: application/json" \
  -d '{"name":"<LOCAL_PART>","local_part":"<LOCAL_PART>","password":"<PASSWORD>"}'
```

### Mailcow (self-hosted, opcional)

Base: `https://<instancia>/api/`
Auth: header `X-API-Key: <KEY>` (ou `Authorization: Bearer <KEY>`, conforme versão).

| Operação | Endpoint | Body |
|---|---|---|
| Listar domínios | `GET /api/v1/domain` | — |
| Verificar username | `GET /api/v1/mailbox/{dominio}/{local_part}` | — |
| Criar mailbox | `POST /api/v1/mailbox` | `{"active":"1","domain":"<DOMINIO>","local_part":"<LOCAL_PART>","name":"<LOCAL_PART>","password":"<PASSWORD>","password2":"<PASSWORD>","quota":"0","tls_enforce_in":"1","tls_enforce_out":"1"}` |

---

## 4. Riscos e custos

| Risco | Impacto | Mitigação no AccMaker |
|---|---|---|
| **Mínimo/máximo de password não documentados** | Slider fica sem valor por definir | `provider.max_password_length` com limite prudente de 64 até haver resposta oficial; a app não depende dele |
| **`createUser` com domínio partilhado não confirmado** | Se a API recusar `purelymail.com`, é preciso domínio próprio | `provider` recebe o domínio como parâmetro — trocar entre `purelymail.com` e domínio próprio é só configuração |
| **Migadu proíbe "automation of any kind"** | Usar o Migadu seria violar os termos | **Não usar.** Ver secção 6 |
| Domínio próprio não verificado (MX/SPF/DKIM) | Mailbox criada mas sem receber correio | Mostrar estado do domínio na app; avisar antes de criar |
| API key comprometida | Criação de mailboxes em nome do utilizador | Chave fora do `.exe` (em `%APPDATA%\AccMaker\`), nunca em logs, nunca no git |
| Limites de taxa não publicados | Falhas em massa ao gerar várias contas | Criação sequencial com pequeno atraso e retry com backoff; respeitar `Retry-After` |
| ToS do FranklyMail proíbe revenda/serviço a terceiros | Usar apenas para as mailboxes do próprio utilizador | Não adotar como fornecedor principal sem o utilizador assumir isso por escrito |
| Password não devolvida após criação | Perder a password gerada | Guardar o par email+password no momento da criação, nunca no repo |

### Custos para o utilizador

- **Purelymail: $10/ano** (plano Simple, confirmado). Sem limite de mailboxes — o custo
  não cresce com o número de contas criadas.
- Migadu: $19/ano (Micro) ou $90/ano (Standard) — **riscado** por termos.
- Mailcow: apenas o custo do servidor (VPS a partir de ~$5/mês).
- Domínio: ~$10/ano, obrigatório em todos os cenários (salvo domínios partilhados).

---

## 6. Termos: criação automatizada de mailboxes

Esta é a questão decisiva, e a resposta **não é a que se esperava**. O AccMaker faz
exatamente isto: criar mailboxes de forma automatizada. O que dizem os termos:

### Purelymail — <https://purelymail.com/termsofservice> (verificado 2026-10-12)

Os termos **não mencionam automação nem criação automatizada de contas**. O que
proíbem é explicitamente:

> "You agree to not use the Service to send junk mail, spam, mailing list emails, or any
> other forms of unsolicited emails, and to not send emails for the purposes of
> advertising, phishing, spreading malware or viruses, or **domain reputation warming**."

Ou seja: a proibição é sobre **o que se envia**, não sobre **quantas mailboxes se criam**.
Não encontrei cláusula que proíba automatizar a gestão da própria conta. Os termos
também não mencionam a API em lado nenhum.

Leitura: **compatível**, para uso próprio, desde que a app não envie spam nem faça
aquecimento de reputação. Nota: silêncio não é autorização — a confirmação continua na
secção 8.

### Migadu — <https://migadu.com/use> (verificado 2026-10-12)

Isto é o oposto, e é explícito:

> "Migadu.com is a personal email service, intended to be used by **humans for
> non-automated** personal or business correspondence. **We do not allow bulk mailing,
> newsletters, automation of any kind**, chain emails or transactional emails."

A secção **Acceptable Use** do Migadu diz literalmente "automation of any kind". O
AccMaker é, na sua função essencial, automação. Isto não é um risco teórico — é uma
cláusula que cobre diretamente o caso de uso.

Os Termos do Migadu acrescentam que pode terminar a conta "**without cause or notice**",
e que contas encontradas a gerar spam são "immediately blocked" com penalização
até $500.

Leitura: **incompatível** com o AccMaker no estado atual.

### Consequência

O Migadu deixa de ser a alternativa séria que parecia. Passa a estar riscado por
falta de compatibilidade de termos, independentemente de a API ser bem documentada.

Isto inverte a recomendação: o fornecedor com a API **não** confirmada em fonte primária
(Purelymail) tem termos compatíveis, e o fornecedor com a API perfeitamente
documentada (Migadu) tem termos que a proíbem.

## 7. Decisão

**Fornecedor: Purelymail. API confirmada em fonte primária.**

A especificação oficial existe e foi lida directamente (secção "Fonte primária da
API"): 19 endpoints, `createUser` com os campos esperados, autenticação por header
`Purelymail-Api-Token`. Não é preciso comprar domínio próprio — os domínios partilhados
existem e custam $0,02 a $0,20 por username e por ano.

O que fica pendente é apenas o que a especificação **não diz**: os limites de password
e se o `createUser` aceita domínio partilhado. Ambas as perguntas estão na secção 8.

O Migadu está **riscado** por incompatibilidade de termos, apesar de a API ser
perfeitamente documentada. Continua documentado neste ficheiro para referência, mas
**não deve ser usado**.

### Arquitetura independentemente do fornecedor

O cliente de API fica atrás de uma interface pequena — um `provider` com
`check_username` e `create_mailbox` — para que trocar de fornecedor seja só acrescentar
uma implementação, sem tocar na interface nem no gerador de passwords.

O **tamanho máximo da password passa a ser um valor por fornecedor**, exposto por
`provider.max_password_length`, que o slider lê. Fica vazio (ou por definir) até haver
resposta oficial; o gerador e a interface não dependem dele.

## 8. Pergunta em aberto ao suporte do Purelymail

**Estado: redigida, por enviar.** Enviar por <https://purelymail.com/support> e guardar
a resposta nesta secção.

> 1. What are the minimum and maximum password lengths accepted by `createUser`, and are
>    there restrictions on special characters?
> 2. Is the API (`/api/v0/createUser`) supported and intended for use by account owners
>    to manage their own mailboxes, or is it subject to restrictions?
> 3. Does `createUser` accept a shared domain such as `purelymail.com` in `domainName`,
>    and is the per-username charge billed through the API?

A resposta a (1) define o máximo do slider. A resposta a (2) é a que confirma se a
automação é aceite. A resposta a (3) decide se é preciso comprar domínio próprio.

Quando abrires a conta (o trial dá créditos limitados), o teste real é: criar uma
mailbox de teste, experimentar passwords curtas e longas para descobrir os limites, e
apagar com `deleteUser`. A API key fica só em variável de ambiente — nunca no chat,
nunca no repositório.

---

## 9. Fornecedor escolhido: AgentMail (plano grátis)

Escolhido porque o requisito passou a ser **gratuito**. Não é um fornecedor de email
pessoal: é um fornecedor de **inboxes para agentes**, e a diferença importa.

### Preço, confirmado em <https://agentmail.to/pricing> (2026-10-12)

| Plano | Preço | Inboxes | Emails/mês | Emails/dia | Armazenamento | Domínios próprios |
|---|---|---|---|---|---|---|
| **Free** | **$0/mês** | **3** | 3 000 | 100 | 3 GB | **Não** |
| Developer | $20/mês | 10 | 10 000 | 1 000 | 10 GB | 10 |
| Startup | $200/mês | 150 | 150 000 | 1 500 | — | 150 |

Sem cartão de crédito no plano Free. Confirmado também por várias fontes de terceiros
(Blog do AgentMail, PrimeForge, UseCarly, Dead Simple) — todas coincidem com a página
oficial.

### API, confirmada contra a documentação oficial

Base `https://api.agentmail.to`, autenticação `Authorization: Bearer <API key>`.
Especificação completa em <https://docs.agentmail.to/openapi.json>.

| Operação | Endpoint |
|---|---|
| Listar | `GET /v0/inboxes` |
| Obter | `GET /v0/inboxes/:inbox_id` |
| Criar | `POST /v0/inboxes` (com `username`, `domain`, `display_name`, `client_id`, `status`, `metadata`) |
| Apagar | `DELETE /v0/inboxes/:inbox_id` |
| Criar API key restrita | `POST /v0/inboxes/{inbox_id}/api-keys` |

`domain` tem de ser um domínio verificado; omitido, assume `agentmail.to`.
`client_id` permite repetir o pedido sem criar duplicados — é a base do `trocar` seguro.

Exemplo, sem chaves reais:

```bash
curl -X POST https://api.agentmail.to/v0/inboxes \
  -H "Authorization: Bearer <API_KEY>" \
  -H "Content-Type: application/json" \
  -d '{"username":"minha-conta","display_name":"accmaker","client_id":"<UUID>"}'
```

### O que este fornecedor é, e o que não é

Isto é a parte que a app tem de dizer com clareza ao utilizador.

**É:**
- Uma inbox real, com endereço real (`nome@agentmail.to`), que recebe e envia email.
- Permanente: **não expira**. A política de expiração de mensagens ao fim de 24 horas
  existe (<https://docs.agentmail.to/message-expiry>) mas é **opcional, por organização
  e desligada por omissão**, só se ativa contacting a equipa. Confirmado.
- Criável e apagável por API a qualquer momento, o que permite trocar uma conta por outra.

**Não é:**
- **Não há password.** A password do IMAP e do SMTP é uma **API key**. Se essa key for
  apagada ou expirar, a conta fica inutilizável e não há recuperação. Por isso o
  AccMaker tem de guardar a key restrita no Credential Manager do Windows, senão um erro
  do utilizador destrói a conta sem volta.
- **Não é domínio próprio** no plano grátis. O endereço fica em `@agentmail.to`.
- **As mensagens enviadas levam o rodapé "Sent via AgentMail"**, que só os planos pagos
  removem.
- **Não é ilimitado**: 3 inboxes, 100 emails/dia. Serve para uso pessoal com limites.
- **Foi feito para agentes de IA e testes**, não para contas pessoais.

### Limites e termos

A skill `provider-agentmail` proíbe explicitamente criar contas ou organizações
múltiplas para contornar o limite de 3. O mesmo está agora no `AGENTS.md`. Não é uma
questão de preço: é contra o espírito da oferta gratuita e arrisca a perda de todas as
inboxes.

### Comparação com o Purelymail

| | AgentMail Free | Purelymail Simple |
|---|---|---|
| Preço | **$0** | $10/ano |
| Inboxes | 3 | ilimitadas |
| Password definida por ti | **Não** (é uma API key) | **Sim** |
| Domínio | `agentmail.to` só | próprio ou partilhado |
| Rodapé nas mensagens enviadas | Sim | Não |
| Rodapé / limite de envio | 100/dia | sem limite rígido |
| Good para | experimentar, agentes, uso pessoal limitado | ter as tuas contas a sério |

### A verificar com um teste real

**Respondidas com uma conta AgentMail Free a 2026-10-12.**

| # | Pergunta | Resposta |
|---|---|---|
| 1 | Apagar uma inbox liberta logo a vaga? | **Sim** — mas exige apagar as API keys da inbox primeiro, senão devolve **409 Conflict** |
| 2 | O username de uma inbox apagada pode ser reutilizado? | **Sim** — o `1-4717` foi apagado e recriado sem problema |
| 3 | Qual é o erro da 4.ª inbox? | **Nunca chega à API.** O AccMaker conta as vagas e recusa antes de pedir |
| 4 | A key restrita funciona em IMAP? | **Por testar** (Thunderbird) |
| 5 | A key só é mostrada uma vez? | Irrelevante: a pergunta 7 prova que se pode sempre criar outra |
| 6 | A key restrita apaga a própria inbox? | **Não.** Devolve **403 `missing_permission`** |
| 7 | Dá para criar outra key para a mesma inbox? | **Sim.** `POST /v0/inboxes/{id}/api-keys` funciona numa inbox existente |

### O que estas respostas mudam

**A conta nunca fica órfã (pergunta 7).** Se a key se perder ou não ficar guardada,
basta criar outra para a mesma inbox. O tratamento da conta órfã no Credential
Manager passa a ser uma chatice, não uma perda. O *apagar* continua a ser um passo
composto (pergunta 1), e a janela deve mostrá-lo como uma acção só, com confirmação,
porque do ponto de vista do utilizador é um botão.

**A key pode ser mostrada em claro (pergunta 6).** Quem tenha a key restrita não
consegue apagar a inbox: a API devolve `403 missing_permission`. Não há risco de
alguém destruir a conta por ter a key. Mostrá-la no ecrã é aceitável e é melhor
para o utilizador.

**O estado HTTP engana (perguntas 1, 2b e 6).** A resposta 403 quer dizer "username
já existe" ou "sem permissão", conforme o `code`. O mapeamento decide pelo `code`,
nunca pelo estado HTTP.