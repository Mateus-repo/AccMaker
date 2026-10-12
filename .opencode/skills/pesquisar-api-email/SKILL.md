---
name: pesquisar-api-email
description: Pesquisa na web e compara APIs oficiais para criar mailboxes de email reais e permanentes (nunca temporárias) e recomenda a melhor para o AccMaker. Usar no início do projeto ou ao trocar de fornecedor.
---

# Pesquisar API de criação de email

## Objetivo
Encontrar uma API que permita **criar programaticamente uma mailbox real e permanente**
(email + password) para o AccMaker, e escrever uma recomendação fundamentada.

## Regras
- **Excluir** serviços de email temporário/descartável (mail.tm, Guerrilla Mail, 10MinuteMail, Temp-Mail, etc.).
- **Excluir** abordagens que automatizam sign-up em fornecedores públicos (Gmail, Outlook.com, Yahoo…) ou contornam CAPTCHA/SMS.
- Só considerar APIs **oficiais e documentadas** de gestão de mailboxes/utilizadores.
- Usar a pesquisa web para confirmar preços, limites e documentação **atuais** — não confiar na memória.
- Registar o URL da documentação oficial de cada candidato.

## Candidatos a verificar (confirmar se ainda existem e o que oferecem)
- **Mailcow** (self-hosted) — API REST para criar mailboxes
- **Migadu** — API de gestão de mailboxes em domínio próprio
- **Purelymail** — API para criar utilizadores/mailboxes
- **Zoho Mail** (organização) — API de criação de utilizadores
- **Google Workspace** — Admin SDK Directory API (requer organização/domínio)
- **Microsoft 365** — Microsoft Graph (criar utilizadores + licença; requer tenant)
- **Forward Email**, **cPanel UAPI**, **iRedMail/Mail-in-a-Box** como alternativas
- Pesquisar também por outros que apareçam (ex.: "email hosting API create mailbox").

## Critérios de comparação
1. Cria mailbox real com password definida pelo chamador? (sim/não)
2. Domínio próprio obrigatório? Custo mensal mínimo / por mailbox
3. Limite máximo/mínimo de password e caracteres aceites (importante para o slider de tamanho)
4. Autenticação (API key, OAuth) e facilidade de usar num .exe sem servidor próprio
5. Limites de taxa e de mailboxes por plano
6. Verifica disponibilidade do username antes de criar?
7. Qualidade da documentação e exemplos
8. Termos de serviço compatíveis com criação automatizada

## Saída obrigatória
Criar `docs/pesquisa-api-email.md` com:
- Tabela comparativa (candidato × critérios)
- **Recomendação** (1.º lugar) + alternativa
- Endpoint(s) exatos para: listar domínios, verificar username, criar mailbox
- Exemplo de pedido (`curl`) com placeholders — **nunca** chaves reais
- Riscos e custos

Depois: aplicar `progresso` e `auto-commit`.
