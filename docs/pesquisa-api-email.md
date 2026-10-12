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
| Custo | **$10/ano flat** (plano Simple), sem limite de users/domínios | $19/ano (Micro) a $29/mês (Standard); mailboxes ilimitadas | $9/ano, $0,40/GB/yr acima de 10 GB | Grátis (custo é o servidor) | Grátis (custo é o servidor) | ~$1,20/utilizador/mês (Mail Lite) | $7/utilizador/mês (Business Starter) | ~$6/utilizador/mês | ~$3–40/utilizador/mês |
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

Ressalva: a documentação pública em `purelymail.com/docs` é sobretudo de cliente de
email; a especificação da API está disponível como swagger no endpoint da API. Convém
fixar o payload validado com um teste real antes de depender dele.

### Alternativa — Migadu

Docs: <https://migadu.com/api> · Preços: <https://migadu.com/pricing>

API REST classical, HTTP Basic (email da conta + API key), referência completa com
exemplos `curl`. Mailboxes ilimitadas em todos os planos, Micro a $19/ano. É a opção
mais conservadora e a melhor se a documentação do Purelymail não estiver acessível.
Ressalva: a própria Migadu descreve a API como "still in early beta".

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
| **API do Purelymail é "beta" / não documentada no site** | Endpoint ou payload podem mudar | `--dry-run` obrigatório; camada de cliente isolada; validar com uma chamada real antes de usar |
| **Mínimo de password não documentado** | Gerador de 6 chars pode ser rejeitado | Definir slider com mínimo conservador (8) e testar empiricamente os limites antes de fixar |
| Domínio próprio não verificado (MX/SPF/DKIM) | Mailbox criada mas sem receber correio | Mostrar estado do domínio na app; avisar antes de criar |
| API key comprometida | Criação de mailboxes em nome do utilizador | Chave fora do `.exe` (em `%APPDATA%\AccMaker\`), nunca em logs, nunca no git |
| Limites de taxa não publicados | Falhas em massa ao gerar várias contas | Criação sequencial com pequeno atraso e retry com backoff; respeitar `Retry-After` |
| ToS do FranklyMail proíbe revenda/serviço a terceiros | Usar apenas para as mailboxes do próprio utilizador | Não Adoptar como fornecedor principal sem o utilizador assumir isso por escrito |
| Mecânica de definição da `password` após criação | Migadu/Purelymail não devolvem a password | Guardar o par email+password localmente no momento da criação, nunca no repo |

### Custos para o utilizador

- Purelymail: $10/ano (≈ €9/ano), flat, sem limite de mailboxes.
- Migadu: $19/ano (Micro) ou $90/ano (Standard).
- Mailcow: apenas o custo do servidor (VPS a partir de ~$5/mês).
- Domínio: ~$10/ano, obrigatório em todos os cenários (salvo domínios partilhados).

---

## 5. Decisão

Adotar **Purelymail** como fornecedor principal, com **Migadu** como alternativa
documentada. O cliente de API deve ser escrito contra uma interface pequena
(`listarDominios`, `verificarUsername`, `criarMailbox`) para que trocar de fornecedor
seja só acrescentar uma implementação, sem tocar na interface nem no gerador de
passwords.