# DevAssist CLI

> Assistente de terminal polyglot em **Rust + Ruby + Lua**.  
> Laboratório de engenharia de IA **100% offline** (sem chave de API).  
> SDD, RAG, Agents, Skills, Workflows RFC e defesas contra prompt injection.  
> Spec-Driven (Spec Kit) + skills de agente no Cursor.

Repositório: [silvalnk/ai-devassist-cli](https://github.com/silvalnk/ai-devassist-cli)  
Pasta local: `devassist_cli/` · Marca: **DevAssist CLI**

![DevAssist CLI: help, plugin Lua, RAG e security audit](docs/images/cli.png)

| | |
|--|--|
| Core | Rust (CLI `devassist`, vector store, host Lua via `mlua`) |
| Agents | Ruby (think → plan → act → observe, RFC HITL) |
| Plugins | Lua 5.4 (skills e prompt templates) |
| IA | Embeddings e LLM via stubs determinísticos |
| Fora de escopo | APIs pagas, UI web, nuvem, Qdrant/Postgres |

## SDD (comece por aqui)

| Arquivo | Papel |
|---------|--------|
| [`.specify/SPEC.md`](.specify/SPEC.md) | **O quê** |
| [`.specify/PLAN.md`](.specify/PLAN.md) | **Como** |
| [`.specify/CONTEXT.md`](.specify/CONTEXT.md) | Estado atual |
| [`AGENTS.md`](AGENTS.md) | Briefing do agente |
| [`docs/GLOSSARY.md`](docs/GLOSSARY.md) | Termos para iniciante |
| [`docs/PLAN.md`](docs/PLAN.md) | Plano + fases |
| [`docs/PRD.md`](docs/PRD.md) | PRD |
| [`.specify/COMMITS.md`](.specify/COMMITS.md) | Conventional commits |
| [`.cursor/skills/README.md`](.cursor/skills/README.md) | Skills do Cursor |
| [`docs/images/cli.png`](docs/images/cli.png) | Print da CLI |

Se código e spec divergirem, a **spec manda**.

## Pré-requisitos

- Rust (`cargo`)
- Ruby >= 3.0
- Git

## Como rodar

```bash
cd devassist_cli

# Build + testes Rust
cd core && cargo test && cargo build

# Plugin Lua
cargo run -- execute ../lua/plugins/hello.lua

# RAG
cargo run -- rag index ../docs/
cargo run -- ask "por que usamos Lua?"

# Prompt templates
cargo run -- prompt benchmark ../lua/templates/

# Segurança
cargo run -- security audit

# Agente Ruby
cd ../agents && ruby run.rb "pesquise RAG e gere um ADR"

# Workflow RFC (pede aprovação humana)
ruby workflow.rb rfc "adicionar streaming"
```

Textos da CLI: inglês. Docs de ensino: português. Sem chave de API.

| Comando | O que faz |
|---------|-----------|
| `execute` | Carrega e roda um plugin Lua |
| `rag index` / `ask` | Indexa markdown e recupera chunks |
| `prompt benchmark` | Mede templates de prompt |
| `security audit` | Evals de prompt injection |
| `ruby run.rb` | Loop think → plan → act → observe |
| `ruby workflow.rb rfc` | RFC com gate humano |

## Arquitetura

```
devassist_cli/
  AGENTS.md
  .specify/
  .cursor/skills/
  docs/images/cli.png
  docs/               # PRD, GLOSSARY, PLAN, ADRs, Design Doc, RFCs
  core/               # CLI Rust + vector store + Lua host
  lua/plugins/        # hello.lua
  lua/templates/      # prompt templates
  agents/             # Agents + Skills + Workflows (Ruby)
  tests/security/     # Payloads extras
```

Rust é o núcleo (`clap`, store, `mlua`). Ruby orquestra agentes e o RFC. Lua entra como plugin e template. O store gerado fica em `core/vector_store.json` (gitignored).

## Licença

Uso educacional.
