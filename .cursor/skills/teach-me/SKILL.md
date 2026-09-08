---
name: teach-me
description: Explica o DevAssist CLI e termos de engenharia de IA para iniciantes em português simples. Use quando o usuário perguntar o que é SDD, PRD, ADR, RAG, skill, agent, RFC, HITL, embedding, chunking ou prompt injection, ou como uma fase deste lab funciona.
---

# Me ensina (iniciante)

Responda em português. Mantenha analogias. Não misture `rag_mnemo` a menos que o usuário peça comparação.

## Fontes (leia se precisar)

- `AGENTS.md` + `.specify/CONTEXT.md` — estado atual (não depende do chat)
- `docs/GLOSSARY.md` — **única fonte de definições** (não reinventar termos)
- `docs/PLAN.md` — fase ↔ conceito
- `.specify/SPEC.md` — capacidades oficiais
- `README.md` — como rodar

## Mapa de ensino

| Conceito | Analogia neste lab |
|----------|-------------------|
| **SPEC.md** | Planta da casa (o quê) |
| **PRD** | Briefing do produto |
| **ADR** | Ata da decisão técnica |
| **RAG / ask** | Folhear o caderno dos docs |
| **Skill Lua** | Plugin / ferramenta no painel (host `mlua`) |
| **Skill Ruby** | Ferramenta do agente (`ResearchSkill`, …) |
| **Agent** | Loop think → plan → act → observe (**está no escopo**) |
| **RFC + HITL** | Assembleia antes de quebrar parede (`[y/n]`) |
| **Stub** | LLM de mentira com a mesma interface |
| **Cursor skill** | Receita para o agente do editor (`.cursor/skills/`) — não é skill Lua/Ruby |

Se perguntarem um termo: abra o GLOSSARY, use **Em uma frase + analogia + Neste projeto**. Não copie o glossário inteiro.

Se perguntarem “em que fase estamos?”: leia CONTEXT.md e aponte a fase em `docs/PLAN.md`.
