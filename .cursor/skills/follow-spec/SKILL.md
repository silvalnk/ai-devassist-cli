---
name: follow-spec
description: Trata os arquivos Spec Kit do DevAssist CLI como fonte da verdade. Use ao editar este repo, implementar fases, mudar CLI/Lua/Ruby, ou quando o usuário citar SDD, spec, CAP, DevAssist ou devassist_cli.
---

# Seguir a spec (devassist_cli)

## Antes de qualquer mudança de código

1. Ler `.specify/CONTEXT.md` (estado atual — sobrevive a uma sessão nova do Cursor).
2. Ler `.specify/SPEC.md` (o que construir).
3. Ler `.specify/PLAN.md` e `docs/PLAN.md` (como / fases).
4. Se código e spec discordarem, **mude o código** ou proponha atualizar a spec primeiro — nunca viole a spec em silêncio.
5. **Depois de qualquer mudança de comportamento, atualize os markdowns no mesmo turno.** Não deixe docs defasados.

Também leia `AGENTS.md` na raiz do repo.

## Idioma

- Markdown: **português**
- Código (comentários, CLI, erros, identificadores): **inglês**
- Mensagens de git: **inglês** (ver `COMMITS.md`)

## Manter markdowns sincronizados (obrigatório)

Quando mudar CLI, API Lua, RAG, agentes Ruby, workflows, nomes de arquivo ou mensagens, atualize **na mesma tarefa**:

| Arquivo | Atualizar quando |
|---------|------------------|
| `.specify/SPEC.md` | Contrato mudou (CAPs, sucesso, CLI, limites) |
| `.specify/PLAN.md` | Arquitetura, módulos ou tarefas Spec Kit mudaram |
| `.specify/CONTEXT.md` | **Sempre** — snapshot ao vivo |
| `docs/PLAN.md` | Fases, diagrama ou entregáveis mudaram |
| `docs/PRD.md` | Requisitos de produto mudaram |
| `docs/GLOSSARY.md` | O ponteiro “Neste projeto” de um termo mudou |
| `AGENTS.md` | Comandos, API ou regras do agente mudaram |
| `README.md` | Como rodar / estrutura |
| `.specify/COMMITS.md` | Só se a convenção de commit mudar |
| `.cursor/skills/*/SKILL.md` | Receitas do agente não batem mais com o código |

A mudança **não está pronta** até o CONTEXT.md bater com o código. Se o usuário não pediu mudança de spec mas o comportamento mudou, atualize a spec mesmo assim (ou proponha o edit primeiro se isso violaria um CAP).

## Escopo

- Stack: núcleo Rust + agentes Ruby + plugins Lua. Sem APIs pagas.
- Embeddings e LLM: stubs determinísticos.
- Binário: `devassist` em `core/`.
- Capacidades: CAP-1 CLI, CAP-2 runtime Lua, CAP-3 RAG, CAP-4 agentes Ruby, CAP-5 RFC/HITL, CAP-6 templates de prompt, CAP-7 segurança.
- Fase atual: CAP-1…7 implementados. Ver CONTEXT.md.

## Fora de escopo (não adicionar)

Deploy em nuvem, UI web, fine-tuning, Qdrant/Postgres, OpenAI/Anthropic, linguagens além de Rust, Ruby, Lua.

## Não confundir com rag_mnemo

Mnemo exclui Agent/RFC. Este repo **inclui**. Não copie os non-goals do Mnemo para cá.

## Docs para humanos

- `README.md`
- `docs/GLOSSARY.md`
- `docs/PLAN.md`
- `.specify/CONTEXT.md`
- `.specify/COMMITS.md`
