# AccMaker

Criador rápido de contas de email — **um único .exe, leve e sem dependências**.

Escolhes o domínio, o username e o **tamanho máximo da password**; o AccMaker gera uma
password forte, cria a mailbox real (permanente) através da API do teu fornecedor de email
e entrega-te `email + password` prontos a copiar.

## Funcionalidades
- Gerador de password com RNG criptográfico: tamanho (mín–máx), maiúsculas, minúsculas, números, símbolos, excluir caracteres ambíguos (`0O1lI`).
- Username manual ou aleatório (legível), com verificação de disponibilidade antes de criar.
- Criação via API oficial de um fornecedor de **mailboxes permanentes** (sem emails temporários).
- Respeita os limites do fornecedor (tamanho máximo de password, caracteres aceites).
- Copiar email / password / ambos com um clique; exportar lote (CSV) opcional.
- Modo `--dry-run` (gera tudo mas não cria nada) para testar sem gastar quota.
- Ficheiro único: configuração e API key guardadas fora do .exe (gestor de credenciais / `%APPDATA%`).

## Requisitos de design
| Requisito | Alvo |
|---|---|
| Distribuição | 1 ficheiro `.exe`, sem DLLs nem pastas |
| Tamanho | o mais pequeno possível (alvo: < 5 MB; ideal < 2 MB) |
| Arranque | instantâneo, sem instalação |
| Segurança | passwords só em memória; segredos nunca no repositório |

## Fornecedor de email

**AgentMail, plano grátis** — 3 inboxes, sem cartão de crédito. Detalhes, preços e
limites em `docs/pesquisa-api-email.md`; instruções de integração na skill
`provider-agentmail`.

O que isso significa na prática:

- A inbox **não expira**. Podes apagá-la e criar outra a qualquer momento.
- **Não há password.** A password do IMAP/SMTP é uma API key, e o AccMaker mostra essa
  key. Se a perderes, a conta fica inutilizável — por isso o AccMaker guarda-a no
  Credential Manager do Windows.
- O endereço é `nome@agentmail.to`. No plano grátis não há domínio próprio.
- As mensagens **enviadas** levam o rodapé "Sent via AgentMail".
- Limite: **3 inboxes**, 100 emails por dia, 3 000 por mês, 3 GB.

> **Os limites do plano grátis, em resumo.** Três inboxes, e é o que dá. A partir da
> 4.ª, a API recusa — o AccMaker diz-te "apaga uma conta primeiro" e não tenta
> contornar. Os 100 emails por dia e os 3 000 por mês **não affectam a criação de
> inboxes**, só o envio. Se algum dia precisares de mais, há o plano Developer a
> $20/mês com 10 inboxes, ou o Purelymail a $10/ano com mailboxes ilimitadas.

**Purelymail** fica disponível como opção paga ($10/ano, mailboxes ilimitadas, com
password escolhida por ti) atrás da mesma interface `provider`. É o caminho se um dia
o limite de 3 deixar de chegar.

> ⚠️ Esta ferramenta cria inboxes num fornecedor de API. Não contorna verificações de
> serviços públicos (Gmail, Outlook, etc.), não usa emails descartáveis e não cria
> contas múltiplas para contornar limites de plano.

## Desenvolvimento com opencode
Este repo traz `AGENTS.md` e skills em `.opencode/skills/`. Abre o opencode na raiz e pede, por exemplo:
> "Segue o AGENTS.md: começa pela skill pesquisar-api-email."

Estado do projeto: ver [`progresso.md`](progresso.md).

## Licença
Ver `LICENSE`.
