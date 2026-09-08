# Cursor Agent Skills (este repo)

Skills do **agente do Cursor** (arquivos `SKILL.md`). Não confundir com skills **Lua** (`register_skill`) nem skills **Ruby** (`ResearchSkill`).

## Como usar no chat

Mencione a skill pelo `name`, ou peça a tarefa que o `description` descreve.

| Skill | Quando |
|-------|--------|
| `conventional-commits` | Pedido de **commit** / commit por arquivo (`✨ feat:` …) |
| `follow-spec` | Qualquer mudança neste projeto (lê CONTEXT + spec; **atualiza os markdowns** no mesmo passo) |
| `teach-me` | Explicar conceitos para iniciante (usa o GLOSSARY) |

## Onde ficam

`.cursor/skills/<nome>/SKILL.md` — só este repositório.
