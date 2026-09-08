# ADR 004 — CLI + JSON como ponte Ruby↔Rust

- Status: Aceito
- Data: 2026-09-08

## Contexto

A spec proíbe gRPC. As skills Ruby ainda precisam dos hits de RAG estruturados vindos do núcleo Rust.

## Decisão

Ruby invoca `devassist ask --json "<question>"` (e `rag query --json`). O stdout é uma resposta JSON-RPC 2.0:

```json
{"jsonrpc":"2.0","result":{"hits":[{"text":"...","source":"...","score":0.2}]},"id":1}
```

Não há daemon RPC de longa duração.

## Consequências

- Fácil de depurar (é só um CLI).
- Cada consulta sobe o binário (aceitável no lab).
- Se o binário não existir, `RAGSkill` cai num stub.

## Alternativas consideradas

- JSON-RPC persistente em stdin/stdout — mais peças móveis para iniciante.
- Sockets Unix / gRPC — fora da spec.
