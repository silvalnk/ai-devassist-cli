# ADR 001 — Rust no núcleo do CLI

- Status: Aceito
- Data: 2026-09-08

## Contexto

O DevAssist precisa de um CLI rápido, um runtime Lua no mesmo processo e um vector store sem APIs pagas.

## Decisão

Usar **Rust** (edition 2021) no binário `devassist` em `core/`: CLI clap, embeddings stub, busca por cosseno e host `mlua`.

## Consequências

- Um binário nativo cobre CAP-1, CAP-2, CAP-3, CAP-6 e CAP-7.
- Ruby fala com Rust via CLI + JSON, não numa VM compartilhada.
- Quem contribui precisa ter `cargo` instalado.

## Alternativas consideradas

- Go ou C++ no núcleo — embedding de Lua mais fraco que `mlua`.
- CLI só em Ruby — FFI mais lento para Lua e álgebra de vetores.
