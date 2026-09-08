# DevAssist CLI — especificação

> Formato Spec Kit (Intenção, Capacidades, Limites, Fora de escopo, Sinal de sucesso)

## Intenção

Engenheiros iniciantes em IA precisam de um ambiente prático para aprender SDD, ADR, RAG, Agents, Skills, Workflows e segurança de prompts — sem depender de APIs pagas. O DevAssist CLI existe para ensinar esses conceitos construindo um assistente de terminal real, polyglot (Rust + Ruby + Lua), 100% offline com stubs.

## Capacidades

- **CAP-1** — CLI Rust com subcomandos `execute`, `rag`, `ask`, `prompt`, `security`
  - **sucesso:** `cargo run -- --help` lista todos os subcomandos
- **CAP-2** — Runtime Lua embeddado via `mlua` com API `log`, `register_skill`, `rag_query`
  - **sucesso:** `cargo run -- execute lua/plugins/hello.lua` registra skill e imprime log
- **CAP-3** — Pipeline RAG (indexar markdown → embeddings stub → busca por similaridade)
  - **sucesso:** `cargo run -- rag index docs/` e `cargo run -- ask "pergunta"` retornam chunks
- **CAP-4** — Agentes Ruby com loop think → plan → act → observe e Skills
  - **sucesso:** `ruby agents/run.rb "pesquise X"` executa skills e imprime observação
- **CAP-5** — Workflow RFC com gate Human-in-the-Loop
  - **sucesso:** `ruby agents/workflow.rb rfc "proposta"` gera RFC e pede aprovação
- **CAP-6** — Prompt templates em Lua + benchmark
  - **sucesso:** `cargo run -- prompt benchmark lua/templates/` lista templates e tempos
- **CAP-7** — Suite de segurança contra prompt injection
  - **sucesso:** `cargo run -- security audit` reporta pass/fail por payload

## Limites / restrições

- Sem OpenAI, Anthropic ou qualquer API paga — apenas stubs determinísticos
- Rust edition 2021+, Ruby >= 3.0, Lua 5.4 via mlua (vendored)
- Comunicação Ruby↔Rust via CLI/JSON (sem gRPC)
- Foco em aprendizado; não é produto de produção

## Fora de escopo

- Deploy em nuvem ou interface web
- Fine-tuning de modelos
- Suporte a outras linguagens além de Rust, Ruby e Lua
- Persistência em Qdrant/Postgres (vector store in-memory basta)

## Sinal de sucesso

Um iniciante consegue clonar o repo, rodar `cargo test` e os comandos CLI/Ruby acima sem chave de API, e explicar cada conceito do glossário apontando o arquivo onde ele foi aplicado.

Uma sessão nova no Cursor lê `AGENTS.md` + `.specify/CONTEXT.md` e recupera o mesmo contrato. Qualquer mudança de comportamento **atualiza no mesmo passo** SPEC/PLAN/CONTEXT/README (e skills do Cursor, se o contrato delas mudar). Commits git seguem [`.specify/COMMITS.md`](./COMMITS.md) (`✨ feat:` …, inglês). Markdown neste repo: nomes SDD em **MAIÚSCULAS**; `README.md` e `SKILL.md` ficam assim por exigência do GitHub/Cursor. Markdown em **português**; código e mensagens de commit em **inglês**.
