# ADR 002 — Lua para plugins e templates de prompt

- Status: Aceito
- Data: 2026-09-08

## Contexto

Quem aprende deve estender o assistente sem recompilar Rust. Templates de prompt também precisam de uma API mínima de `render()`.

## Decisão

Embutir **Lua 5.4** via `mlua` (vendored). Plugins em `lua/plugins/`. Templates em `lua/templates/` expõem `render()`.

API do host: `log`, `register_skill`, `rag_query`.

## Consequências

- Plugins são scripts, não crates.
- Skills registradas em Lua existem só naquele processo.
- Lua não chama APIs pagas; o RAG passa pelo host Rust.

## Alternativas consideradas

- Plugins JavaScript/WASM — runtime pesado demais para o lab.
- Plugins Ruby dentro do CLI — mistura a linguagem do agente com o host.
