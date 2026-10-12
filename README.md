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

Fornecedor com **API oficial de gestão de mailboxes**. A escolha e a comparação estão em
`docs/pesquisa-api-email.md`.

**Purelymail** — plano Simple de **$10/ano**, sem limite de mailboxes nem de domínios.
Não é preciso comprar domínio: os domínios partilhados (como `purelymail.com`) têm uma
taxa anual por username, de **$0,02 a $0,20** conforme o comprimento (13+ caracteres =
$0,02). A API é um `POST` com uma API key num header, 19 endpoints documentados
oficialmente.

Detalhes e fontes em `docs/pesquisa-api-email.md`.

> ⚠️ Esta ferramenta serve para criar contas no teu próprio domínio ou nos domínios
> partilhados do fornecedor. Não contorna verificações de serviços públicos (Gmail,
> Outlook, etc.) nem usa emails descartáveis.

## Desenvolvimento com opencode
Este repo traz `AGENTS.md` e skills em `.opencode/skills/`. Abre o opencode na raiz e pede, por exemplo:
> "Segue o AGENTS.md: começa pela skill pesquisar-api-email."

Estado do projeto: ver [`progresso.md`](progresso.md).

## Licença
Ver `LICENSE`.
