# DevAssist CLI

Assistente de terminal polyglot (**Rust + Ruby + Lua**) para aprender engenharia de IA na prática — **100% offline**, sem chave de API.

## O que você aprende

SDD, PRD, ADR, Design Docs, RFC, Agents, Skills, Workflows, RAG, Prompt Engineering, Prompt Injection e defesas.

- Glossário (estudo termo a termo): [`docs/GLOSSARY.md`](docs/GLOSSARY.md)
- Plano + fases: [`docs/PLAN.md`](docs/PLAN.md)
- Spec (Spec Kit): [`.specify/SPEC.md`](.specify/SPEC.md) · [`.specify/PLAN.md`](.specify/PLAN.md)
- Contexto entre sessões: [`AGENTS.md`](AGENTS.md) · [`.specify/CONTEXT.md`](.specify/CONTEXT.md)
- Commits: [`.specify/COMMITS.md`](.specify/COMMITS.md)
- PRD: [`docs/PRD.md`](docs/PRD.md)
- Agent skills (Cursor): [`.cursor/skills/README.md`](.cursor/skills/README.md)

## Pré-requisitos

- Rust (cargo)
- Ruby >= 3.0
- Git

## Começo rápido

```bash
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

## Estrutura

```
AGENTS.md            # briefing do agente (nova sessão)
.specify/SPEC.md     # Spec (Spec Kit)
.specify/PLAN.md     # How
.specify/CONTEXT.md  # estado atual
.specify/COMMITS.md  # conventional commits + emoji
.cursor/skills/      # follow-spec, conventional-commits, teach-me
docs/                # PRD, GLOSSARY, PLAN, ADRs, Design Doc, RFCs
core/                # CLI Rust + vector store + Lua host
agents/              # Agents + Skills + Workflows (Ruby)
lua/                 # Plugins e prompt templates
tests/security/      # Payloads extras
```

## Licença

Uso educacional.
