# DevAssist CLI — contexto persistente

> Leia este arquivo **no início de qualquer sessão** (Cursor ou clone novo).
> Não depende do chat anterior. A fonte da verdade continua sendo [SPEC.md](./SPEC.md).
> **Quem altera código atualiza este arquivo (e SPEC/PLAN/README/GLOSSARY se o contrato ou o “como rodar” mudarem).**

Markdown neste repo: nomes SDD em **MAIÚSCULAS** (`SPEC.md`, `PLAN.md`, `CONTEXT.md`, `COMMITS.md`, `AGENTS.md`, `PRD.md`). Exceções (ferramentas exigem): `README.md`, `.cursor/skills/*/SKILL.md`.

**Idioma:** markdown em **português**; código (comentários, CLI, erros, identificadores, commits) em **inglês**.

Atualizado para o estado **implementado** (CAP-1…7).

## Identidade

| | |
|--|--|
| Produto | **DevAssist CLI** |
| Pasta local | `devassist_cli/` |
| Binário | `devassist` em `core/` (`cargo run` a partir de `core/`) |
| Stack | Rust core + Ruby agents + Lua plugins. Sem API paga |
| Embeddings / LLM | stubs determinísticos |
| Store | `core/vector_store.json` (gerado; gitignored) |
| Foco | laboratório de aprendizado, não produto de produção |

## O que o lab ensina (fases 1–8) — todas implementadas

1. **SDD + PRD** — spec primeiro, código depois
2. **ADR + Design Doc** — `docs/adr/`, `docs/design/ARCHITECTURE.md`
3. **Plugins / FFI** — `execute` + `lua/plugins/hello.lua`
4. **RAG** — `rag index` / `ask` (hash stub + cosine)
5. **Agents + Skills** — `ruby agents/run.rb` (think → plan → act → observe)
6. **Workflows + RFC + HITL** — `ruby agents/workflow.rb rfc "…"` (`Approve? [y/n]`)
7. **Prompt engineering** — `prompt benchmark lua/templates/`
8. **Segurança** — `security audit` + `sanitize_input` / `validate_output`

## Estado atual

| Artefato | Status |
|----------|--------|
| `.specify/SPEC.md` | CAP-1…7 |
| `core/` | CLI + tests (`cargo test`) |
| `lua/plugins/`, `lua/templates/` | hello, rag_query, 3 templates |
| `agents/` | Agent + 3 skills + RFC workflow |
| `docs/adr/001`…`005` | ADRs |
| `docs/design/ARCHITECTURE.md` | design doc |
| `docs/rfcs/TEMPLATE.md` | template (RFCs numeradas são geradas e gitignored) |
| `tests/security/PAYLOADS.txt` | evals |

## Como rodar

```bash
cd core && cargo test && cargo build
cargo run -- execute ../lua/plugins/hello.lua
cargo run -- rag index ../docs/
cargo run -- ask "por que usamos Lua?"
cargo run -- prompt benchmark ../lua/templates/
cargo run -- security audit
cd ../agents && ruby run.rb "pesquise RAG e gere um ADR"
printf 'n\n' | ruby workflow.rb rfc "adicionar streaming"
```

`ask --json` devolve JSON-RPC 2.0 para o `RAGSkill` Ruby (`agents/lib/bridge.rb`).

## Lua API (CAP-2)

`log`, `register_skill`, `rag_query` (+ `run_skill` no host).

## Fora de escopo (não adicionar)

Deploy em nuvem, UI web, fine-tuning, Qdrant/Postgres, OpenAI/Anthropic, outras linguagens além de Rust, Ruby e Lua.

## Dois tipos de “skill” (não misturar)

- **Cursor** — `.cursor/skills/*/SKILL.md`
- **Lua** — `register_skill` no host `mlua`
- **Ruby** — `ResearchSkill`, `CodeGenSkill`, `RAGSkill`

## Relação com `rag_mnemo`

Mnemo é o lab **mínimo** (sem Agent). DevAssist é o lab **completo** (polyglot + Agent + RFC + segurança).

## Docs para humanos / próxima sessão

| Arquivo | Papel |
|---------|--------|
| [SPEC.md](./SPEC.md) | contrato (o quê) |
| [PLAN.md](./PLAN.md) | como + tasks |
| Este CONTEXT | estado atual |
| [COMMITS.md](./COMMITS.md) | conventional commits + emoji |
| `docs/PLAN.md` | fases |
| `docs/GLOSSARY.md` | definições |
| `AGENTS.md` | regras para o agente Cursor |
