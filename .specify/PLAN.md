# Plano (Spec Kit — como)

Companheiro do [SPEC.md](./SPEC.md). Define *como* vamos construir o DevAssist CLI.

## Stack

- **Rust**: CLI, vector store, host Lua (`mlua`)
- **Ruby**: agents, skills, workflows (stubs, sem API paga)
- **Lua**: plugins e prompt templates

## Fluxo SDD

1. Specify → `.specify/SPEC.md` (já existe)
2. Plan → este arquivo + `docs/PLAN.md` (glossário e fases detalhadas)
3. Tasks → fases 1–8 em `docs/PLAN.md`
4. Implement → código em `core/`, `agents/`, `lua/`
5. Analyze → `cargo test` + comandos de smoke do README

## Restrições

- Zero API key obrigatória
- Embeddings e LLM via stubs determinísticos
- Comunicação Ruby↔Rust via CLI (e JSON quando necessário)

## Documentação principal

- Glossário expandido (iniciantes): **[docs/GLOSSARY.md](../docs/GLOSSARY.md)**
- Plano + fases: **[docs/PLAN.md](../docs/PLAN.md)**
- Contexto entre sessões: [`AGENTS.md`](../AGENTS.md) + [CONTEXT.md](./CONTEXT.md)
- Commits: [COMMITS.md](./COMMITS.md)

## Tarefas (CAP-1…7)

- [x] CLI clap: `execute`, `rag`, `ask`, `prompt`, `security`
- [x] Host Lua: `log`, `register_skill`, `rag_query` + `hello.lua`
- [x] RAG: chunk por títulos, embed stub, cosseno, `vector_store.json`
- [x] Agente Ruby: think → plan → act → observe + 3 skills
- [x] Workflow RFC + HITL `[y/n]`
- [x] Templates de prompt + `prompt benchmark`
- [x] Audit de segurança + payloads
- [x] ADRs 001–005 + `docs/design/ARCHITECTURE.md`
- [x] `cargo test` em máquina limpa (sem API key)

## Idioma

Markdown em português. Código (identificadores, comentários, CLI, erros, commits) em inglês.

