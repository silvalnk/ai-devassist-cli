# DevAssist CLI — documento de requisitos de produto (PRD)

## 1. Propósito

Construir um **CLI polyglot de aprendizado** que ensina engenharia de IA na prática: SDD, ADR, RAG, Agents, Skills, Workflows, Prompt Engineering e segurança — sem APIs pagas.

## 2. Usuários-alvo

| Persona | Necessidade |
|---------|-------------|
| **Learner** | Iniciante em IA que quer entender cada conceito implementando |
| **Contributor** | Dev que estende plugins Lua ou skills Ruby |

## 3. Papel das linguagens

| Linguagem | Papel | Bibliotecas-chave |
|----------|--------|-------------------|
| **Rust** | Núcleo: CLI, vector store, host Lua | `clap`, `mlua`, `serde` |
| **Ruby** | Agentes, skills, workflows | stdlib + stubs |
| **Lua** | Plugins e templates de prompt | hospedado pelo `mlua` |

## 4. Requisitos funcionais

- FR-01: `execute <script.lua>` carrega e executa plugin Lua
- FR-02: `rag index <dir>` indexa markdown no vector store
- FR-03: `ask "<question>"` / `rag query` busca por similaridade
- FR-04: `prompt benchmark <dir>` compara templates Lua
- FR-05: `security audit` roda suite de prompt injection
- FR-06: Agente Ruby com ResearchSkill, CodeGenSkill, RAGSkill
- FR-07: Workflow RFC → Review → Approve → Implement → Verify com gate humano

## 5. Requisitos não funcionais

- NFR-01: Zero dependência de API key
- NFR-02: `cargo test` passa em máquina limpa
- NFR-03: Documentação (spec, PRD, ADRs) acompanha cada decisão
- NFR-04: Markdown em **português**; código (comentários, CLI, erros) em **inglês**

## 6. Entrega por fases

| Fase | Conceito | Entregável |
|-------|----------|------------|
| 1 | SDD, PRD | SPEC.md, PRD.md, scaffold |
| 2 | ADR, Design Doc | docs/adr/*, ARCHITECTURE.md |
| 3 | Plugins, FFI | CLI + runtime Lua |
| 4 | RAG | vector store + ask |
| 5 | Agents, Skills | orquestrador Ruby |
| 6 | Workflows, RFC, HITL | rfc_workflow.rb |
| 7 | Prompt Engineering | templates + benchmark |
| 8 | Segurança | suite de injection + defesas |
