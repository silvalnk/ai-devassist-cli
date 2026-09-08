# AGENTS.md

Este arquivo é o briefing **independente de sessão** para o Cursor (e qualquer agente de código). O histórico do chat é opcional; estes arquivos não.

## Sempre

1. Ler [`.specify/CONTEXT.md`](.specify/CONTEXT.md) (estado atual do produto).
2. Ler [`.specify/SPEC.md`](.specify/SPEC.md) (fonte da verdade).
3. Ler [`.specify/PLAN.md`](.specify/PLAN.md) e [`docs/PLAN.md`](docs/PLAN.md) antes de mudar *como* as coisas são construídas.
4. Se código e spec discordarem, **mude o código** ou proponha atualizar a spec primeiro. Nunca viole a spec em silêncio.
5. Seguir [`.cursor/skills/follow-spec/SKILL.md`](.cursor/skills/follow-spec/SKILL.md) em qualquer mudança de código.
6. **No mesmo turno da mudança de código:** atualizar `.specify/CONTEXT.md` e qualquer spec / plano / PRD / README / glossário / skill de agente que ficaria mentindo. Doc atrasado em relação ao código é violação da spec.
7. Quando o usuário pedir **commit**, seguir [`.specify/COMMITS.md`](.specify/COMMITS.md) e [`.cursor/skills/conventional-commits/SKILL.md`](.cursor/skills/conventional-commits/SKILL.md) (`✨ feat:` / `📝 docs:` / …, **mensagens em inglês**).
8. Ao ensinar iniciantes, seguir [`.cursor/skills/teach-me/SKILL.md`](.cursor/skills/teach-me/SKILL.md). Definições ficam em [`docs/GLOSSARY.md`](docs/GLOSSARY.md).

## Idioma

- **Markdown** (incluindo este arquivo): **português**.
- **Código** (identificadores, comentários, CLI, erros, logs, commits git): **inglês**.
- Perguntas de exemplo nos docs podem ser em português (idioma de quem aprende).

## Manter docs sincronizados

Mudança de comportamento → mudança de markdown. Mínimo: **CONTEXT.md**. Mudança de contrato → **SPEC.md** também. Como rodar → **README.md**. Definições → **GLOSSARY.md**.

Nomes dos markdowns SDD em **MAIÚSCULAS** (`SPEC.md`, `PLAN.md`, `CONTEXT.md`, `COMMITS.md`, `AGENTS.md`, `PRD.md`). Exceções (ferramentas exigem): `README.md`, `.cursor/skills/*/SKILL.md`.

## Produto (um parágrafo)

**DevAssist CLI** é um laboratório polyglot offline (**Rust + Ruby + Lua**) para aprender engenharia de IA na prática: SDD, PRD, ADR, RAG, Agents, Skills, Workflows, prompt engineering e defesas contra prompt injection. Sem API paga. Embeddings e LLM via stubs determinísticos. Binário: `devassist` em `core/`.

## Fase atual

**CAP-1…7 implementados.** Comandos a partir de `core/` (Rust) e `agents/` (Ruby). Atualize este arquivo e o CONTEXT.md se o comportamento mudar.

## Comandos

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

Textos da CLI: inglês. Docs de ensino: português.

## Não adicionar

APIs pagas (OpenAI / Anthropic), deploy em nuvem, UI web, fine-tuning, Qdrant/Postgres, linguagens além de Rust, Ruby e Lua.

## Papel das linguagens

| Linguagem | Papel |
|----------|--------|
| **Rust** | CLI, vector store, host Lua (`mlua`), ponte JSON-RPC |
| **Ruby** | Agentes, skills, workflows RFC (stubs, sem API paga) |
| **Lua** | Plugins e templates de prompt |

Não misturar este repo com `rag_mnemo`. Mnemo é só RAG+Lua e **exclui** Agent/RFC. DevAssist **inclui** loop de Agent e HITL.

## Docs para humanos

- `README.md`
- `docs/GLOSSARY.md` — única fonte de definições
- `docs/PLAN.md` — arquitetura + fases 1–8
- `docs/PRD.md`
- `.specify/COMMITS.md` — mensagens de git (conventional commits + emoji, em inglês)
