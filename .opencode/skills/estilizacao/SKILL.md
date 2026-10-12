---
name: estilizacao
description: Guia de estilo visual da interface do AccMaker (tema escuro, tokens de cor, tipografia, espaçamento, estados e acessibilidade) para uma UI limpa, pequena e rápida. Usar sempre que criares ou alterares a interface.
---

# Estilização do AccMaker

## Princípios
- Janela **pequena e de uma só vista** (~420×560), sem separadores nem menus escondidos.
- Fluxo em 3 passos visíveis: **Domínio/Username → Password → Criar**.
- Resultado grande e legível: email + password com botão **Copiar** ao lado de cada um.
- Estética sóbria. Zero ficheiros de imagem externos: ícones desenhados em código ou
  caracteres.

## Realidade da interface: Win32 nativo

A GUI é **Win32 nativo** (`windows-sys`), não uma biblioteca de widgets. Decisão de
2026-10-12, baseada em medição: o `egui` dá 2,85 MB só com a janela e um botão,
contra 0,11 MB do Win32. Como o alvo é < 2 MB, o `egui` não serve.

Consequência: o que se descreve a seguir é o **alvo visual**, e o que não for nativo
fica aproximado. Não prometer o que não sai.

| Elemento | Nativamente | Notas |
|---|---|---|
| Botões, campos de texto, listas, caixas de verificação | **Sim** | controlos padrão do Windows |
| Tema escuro | **Sim** | `DwmSetWindowAttribute`, Windows 10 1809+ |
| Cantos arredondados | **Sim** | `DwmSetWindowAttribute` com `DWMWCP_ROUND`, Janela 11 |
| Slider do tamanho da password | **Sim** | `TRACKBAR`, com o valor numérico ao lado |
| Botão "Copiar" | **Sim** | escrever na area de transferencia |
| Barra de força colorida | **Parcial** | `PROGRESSBAR` colorido, sem gradiente |
| Animações suaves | **Não** | só as de abrir e fechar que o Windows dá |
| Ícones vetoriais | **Não** | usar caracteres Unicode, sem ficheiros |

Se um efeito não couber na coluna "Sim" ou "Parcial", **aproximar ou deixar de fora**.
Não desenhar controlos à mão só por causa de um canto: custa tempo, aumenta o risco
de bugs, e o tamanho do .exe é o objetivo principal.

## Tokens (tema escuro por omissão, com tema claro opcional)
| Token | Escuro | Claro |
|---|---|---|
| `bg` | `#0F1115` | `#F6F7F9` |
| `surface` | `#171A21` | `#FFFFFF` |
| `border` | `#2A2F3A` | `#DDE1E8` |
| `text` | `#E8EAF0` | `#14171F` |
| `muted` | `#8B93A5` | `#5B6475` |
| `accent` | `#5B8CFF` | `#2F5DE0` |
| `success` | `#3DD68C` | `#1E9E62` |
| `danger` | `#FF5C6C` | `#D63645` |

## Tipografia e espaçamento
- Fonte do sistema (Segoe UI no Windows); **monoespaçada** para email e password.
- Escala: 12 / 14 / 16 / 22 px. Espaçamento em múltiplos de 4 (8, 12, 16, 24).
- Altura mínima de botão: 36 px. Cantos arredondados ao nível da janela, não dos controlos.

## Componentes
- **Slider de tamanho da password** com valor numérico visível e limites (mín–máx) vindos do fornecedor.
- **Interruptores** para maiúsculas/minúsculas/números/símbolos/excluir ambíguos.
- **Indicador de força** (barra com cores `danger` → `accent` → `success`).
- **Botão primário** "Criar conta" (`accent`); desativado com spinner enquanto o pedido corre.
- **Toasts** curtos para "Copiado!" e erros — erros em linguagem humana, nunca stack traces.

## Estados e acessibilidade
- Estados obrigatórios: normal, hover, foco visível, desativado, a carregar, erro, sucesso.
- Contraste mínimo WCAG AA (4.5:1 para texto).
- Navegação completa por teclado (Tab/Enter/Esc); `Ctrl+C` copia o campo focado.
- Password escondida por omissão com botão "mostrar".

## Não fazer
- Sem animações pesadas, sem gradientes complexos, sem fontes ou imagens externas que aumentem o .exe.
