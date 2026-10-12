---
name: build-exe-leve
description: Escolher a linguagem e as flags de compilação para o AccMaker ficar num único .exe muito leve e standalone, e medir o tamanho. Usar ao criar o esqueleto do projeto, adicionar dependências ou preparar o build final.
---

# Build de .exe leve e standalone

## Requisitos
- Um único `.exe`, **sem** DLLs, runtimes, pastas de assets ou instalador.
- Tamanho alvo: < 5 MB (ideal < 2 MB). Reportar o tamanho real em `progresso.md`.

## Opções (decidir e justificar)
| Linguagem | Tamanho típico | Notas |
|---|---|---|
| **Rust** (+ `egui`/`eframe` ou `native-windows-gui`/`windows-sys`) | ~0.5–4 MB | Melhor equilíbrio; sem runtime; HTTP com `ureq` + `rustls` (ou WinHTTP) |
| **Go** (+ UI nativa tipo `walk` ou UI web embebida) | ~5–9 MB (~2–3 MB com UPX) | Muito simples, cross-compile fácil; tudo estático |
| **Zig / C com Win32** | < 1 MB | Mais trabalhoso; só se o tamanho for crítico |
| Evitar | Electron, .NET com runtime, Python+PyInstaller, Java | Pesados ou com dependências |

Recomendação por omissão: **Rust** com GUI simples. Se a UI complicar, começar por CLI/TUI e só depois GUI.

## Rust — perfil de release
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```
Build: `cargo build --release` (Windows: target `x86_64-pc-windows-msvc`, ligar CRT estaticamente com `RUSTFLAGS="-C target-feature=+crt-static"`).

## Go — build
`go build -trimpath -ldflags "-s -w" -o dist/AccMaker.exe` (`CGO_ENABLED=0`).

## Opcional
- `upx --best` para comprimir (avisar que alguns antivírus desconfiam de executáveis UPX; preferir sem UPX se der falsos positivos).
- Ícone/versão embebidos no próprio .exe (resource), nunca como ficheiro à parte.

## Dados e config
- Nada ao lado do .exe. Config em `%APPDATA%\AccMaker\config.toml`; API key no Credential Manager do Windows (ou cifrada com DPAPI).
- Primeira execução: pedir os dados necessários e guardá-los.

## Verificação
1. Compilar em release.
2. Medir: `ls -l`/`Get-Item` no .exe e registar.
3. Copiar **só o .exe** para uma pasta vazia e executar — tem de funcionar.
4. Se subir muito de tamanho após adicionar uma dependência, reavaliar.
