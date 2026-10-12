# AGENTS.md — AccMaker

Instruções para o agente (opencode). Lê este ficheiro **no início de cada sessão**.

## Objetivo do projeto
App **standalone** (um único `.exe`, sem ficheiros ao lado) e **muito leve** que cria
contas de email com password de forma rápida:
o utilizador escolhe o domínio, o username (manual ou aleatório) e o **tamanho
máximo da password**; a app gera uma password forte e cria a **mailbox real e
permanente** através da API de um fornecedor. No fim mostra email + password com botão de copiar.

## Regras gerais
- Responde e comenta em **português europeu (PT-PT)**. Código, nomes de variáveis e commits seguem a convenção abaixo.
- Mailboxes **reais e permanentes**, num domínio do próprio utilizador, num domínio
  partilhado do fornecedor, ou num fornecedor com API oficial.
  **Nunca** usar serviços de email temporário/descartável.
- O `provider` recebe o **domínio como parâmetro**, para poder usar um domínio próprio
  ou um domínio partilhado sem alterar código.
- **Nunca** criar contas ou organizações múltiplas num fornecedor para contornar um
  limite de plano. É contra o espírito da oferta gratuita e arrisca perder tudo.
- **Não implementar** bypass de CAPTCHA, verificação por telemóvel, rotação de IPs ou automação de
  sign-up em Gmail/Outlook/Yahoo e afins. Só APIs oficiais de administração de mailboxes.
- Passwords geradas com **RNG criptográfico** (nunca `rand` normal). Nunca escrever passwords/API keys em logs.
- Segredos (API key) **nunca** vão para o git; ficam no gestor de credenciais do SO ou em `%APPDATA%\AccMaker\` (fora do .exe/pasta do projeto).
- O binário final tem de funcionar **copiado sozinho** para outro PC (sem DLLs, sem pastas de assets, sem instalador).
- Antes de acrescentar uma dependência, justifica o impacto no tamanho do .exe.

## Skills disponíveis (`.opencode/skills/`)
| Skill | Quando usar |
|---|---|
| `pesquisar-api-email` | No início do projeto ou quando for preciso trocar/validar o fornecedor de API de email |
| `provider-agentmail` | Ao implementar ou alterar o fornecedor AgentMail: inboxes, slots, chaves, interface de contas |
| `build-exe-leve` | Escolha da linguagem, flags de compilação e verificação do tamanho do .exe |
| `estilizacao` | Sempre que mexeres na interface (cores, layout, tipografia) |
| `progresso` | **No fim de cada tarefa**: atualizar `progresso.md` |
| `auto-commit` | **No fim de cada tarefa concluída**: commit automático (depois de `progresso`) |

## Fluxo obrigatório por tarefa
1. Ler `progresso.md` para saber o estado atual.
2. Implementar a tarefa (pequena e focada).
3. Compilar e testar (`build-exe-leve` indica como medir o tamanho).
4. Aplicar a skill `progresso` (atualiza `progresso.md`).
5. Aplicar a skill `auto-commit` (commit com mensagem convencional; push só se o remote existir).

## Ordem sugerida do projeto
1. `pesquisar-api-email` → gerar `docs/pesquisa-api-email.md` e propor fornecedor.
2. Escolher linguagem com `build-exe-leve` e criar esqueleto.
3. Gerador de passwords (CLI/teste primeiro, sem rede).
4. Cliente da API (com modo `--dry-run` que não cria nada).
5. Interface (`estilizacao`).
6. Release: .exe final < alvo de tamanho definido em `progresso.md`.

## Escrita (regra estrita)
- Escrever **apenas em português europeu**, sem andar a trocá-lo por inglês no
  texto corrido.
- **Nunca escrever caracteres fora do alfabeto latino** nos ficheiros do projeto:
  nada de caracteres CJK, cirílicos, grego, emoji ou símbolos exóticos. Apenas
  latin supplement + acentuação portuguesa (á, â, ã, ç, é, ê, í, ó, ô, õ, ú).
  Exceção única: código, identificadores e texto de API de terceiros, que se
  escrevem como a fonte original os define.
- Ao concluir um ficheiro, **relê-lo antes do commit** a confirmar que não há
  caracteres estranhos nem texto truncado. Se aparecerem, corrige-os antes de
  commitar — nunca vão para o git.
- Se uma frase soar estranhamente truncada ou a meio, relê-la e reescrevê-la.

## Convenções
- Commits: `tipo(âmbito): descrição` em PT-PT — `feat`, `fix`, `refactor`, `docs`, `chore`, `style`, `test`.
- Um commit por tarefa concluída. Nunca `git push --force`.
- Ficheiros de documentação em `docs/`; estado do projeto em `progresso.md` (raiz).
