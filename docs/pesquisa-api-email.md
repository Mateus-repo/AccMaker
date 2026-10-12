# Pesquisa de API de email — AccMaker

Data: 2026-10-12. Objetivo: encontrar uma API **oficial e documentada** que crie
mailboxes **reais e permanentes** com password definida pelo chamador, a partir de um
`.exe` Windows standalone (sem servidor próprio, sem OAuth browser).

Excluídos por regra do projeto: mail.tm, Guerrilla Mail, 10MinuteMail e restantes
temporários/descartáveis; automação de sign-up em Gmail/Outlook/Yahoo; qualquer bypass
de CAPTCHA/SMS.

---

## 1. Tabela comparativa

| Critério | **Purelymail** | **Migadu** | **FranklyMail** | **Mailcow** (self-hosted) | **Stalwart** (self-hosted) | **Zoho Mail** (org) | **Google Workspace** | **Microsoft 365** | **Fastmail** |
|---|---|---|---|---|---|---|---|---|---|
| Cria mailbox real com password do chamador | **Sim** (`createUser` com `password`) | **Sim** (`POST .../mailboxes` com `password`) | **Sim** (`POST /mailboxes` com `password`) | Sim | Sim (via JMAP `Account/set`) | Sim | Sim | Sim (com licença) | **Não** (sem API de provisioning) |
| Domínio próprio obrigatório | Quase (há domínios partilhados `purelymail.com`) | **Sim** | **Sim** | **Sim** | **Sim** | **Sim** + conta Zoho paga | **Sim** + Workspace | **Sim** + tenant M365 | Não (domínios próprios, mas sem API admin) |
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
| Existência da API `createUser` | **NÃO verificada em fonte primária** | ver abaixo |
| Limites de password | **Desconhecidos** | nenhum |

Ressalva importante: **não existe documentação oficial da API no site do Purelymail.**
Verificado em 2026-10-12:
- `purelymail.com/docs` — índice oficial, **sem qualquer página de API** (só setup,
  domínios, users, routing, filtros, FAQ).
- `purelymail.com/api/v0/swagger.json` — **404**.
- Sem página de API referenciada em qualquer lado do site ou da documentação.

Tudo o que sabemos sobre `createUser` vem de **terceiros**: a extensão oficial da
Raycast e MCP servers da comunidade. É consistente entre si e usa um payload plausível,
mas não é fonte primária e pode estar desactualizado ou errado.

Não escrever o cliente da API contra esta fonte sem antes de obter confirmação
oficial (ver secção 6).

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
| **API do Purelymail não documentada em fonte primária** | Endpoint ou payload podem não existir ou ter mudado | Não escrever o cliente antes da resposta do suporte (secção 8); `--dry-run` sempre; cliente isolado atrás da interface `provider` |
| **Mínimo/máximo de password desconhecidos** | Slider fica sem valor por definir | `provider.max_password_length` fica vazio até haver resposta oficial; a app não depende dele |
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

Leitura: **compatível**, para uso próprio no próprio domínio, desde que a app não envie
spam nem faça aquecimento de reputação.

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

**Fornecedor: Purelymail, pendente de confirmação oficial.**

O Purelymail passa a ser o único candidato com termos compatíveis com o caso de uso.
A API continua **não verificada em fonte primária**, pelo que o cliente só deve ser
escrito depois da resposta do suporte (secção 8).

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

> 1. What are the minimum and maximum password length accepted by `createUser`, and are
>    there any restrictions on special characters?
> 2. Is the API (`/api/v0/createUser`) supported and intended for use by account owners
>    to manage their own mailboxes in bulk, or is it subject to restrictions?

A resposta a (1) define o máximo do slider. A resposta a (2) decide se o Purelymail é
viável.