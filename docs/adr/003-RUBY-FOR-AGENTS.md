# ADR 003 — Ruby para agentes, skills e workflows

- Status: Aceito
- Data: 2026-09-08

## Contexto

A spec exige um loop de agente (think → plan → act → observe), skills nomeadas e um workflow RFC com gate humano. Essa orquestração fica mais legível numa linguagem de script.

## Decisão

Usar **Ruby >= 3.0** (só stdlib) em `agents/`. Chamadas de LLM passam por `StubLLM`. O RAG usa a saída JSON do CLI Rust.

## Consequências

- Sem Gemfile.
- O código do agente é fácil de imprimir e ensinar.
- Dois processos: `ruby` + `devassist`.

## Alternativas consideradas

- Agentes em Rust — mais boilerplate para HITL e para ensinar.
- Agentes em Python — linguagem extra fora da spec (só Rust, Ruby e Lua).
