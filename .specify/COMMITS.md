# Conventional commits + emoji

Fonte da verdade para **mensagens de git** neste repo.  
Skill do agente: `.cursor/skills/conventional-commits/SKILL.md`.  
Na próxima sessão: `/conventional-commits` ou “faça o commit”.

O arquivo é em **português**. As **mensagens de commit** (código/git) permanecem em **inglês**.

## Formato

```
<emoji> <type>: <subject>

[corpo opcional — o *porquê*, não um dump de arquivos]
```

- Idioma da mensagem: **inglês**
- Subject: imperativo, sem ponto final, ~72 caracteres
- Uma mudança lógica por commit **salvo** se o usuário pedir um commit **por arquivo**

## Tipos e emoji

| Tipo | Emoji | Quando usar |
|------|-------|-------------|
| `feat` | ✨ | Comportamento novo (CLI, Lua, agentes Ruby, RAG, segurança) |
| `fix` | 🐛 | Correção de bug |
| `docs` | 📝 | SPEC, CONTEXT, README, GLOSSARY, PRD, PLAN |
| `chore` | 🙈 | gitignore, tooling |
| `chore` | 🔒 | `Cargo.lock` / `Gemfile.lock` |
| `chore` | 🤖 | `.cursor/skills/` |
| `chore` | 📦 | só `Cargo.toml` / `Gemfile` |
| `refactor` | ♻️ | Mesmo comportamento, código mais claro |
| `test` | ✅ | Só testes |
| `perf` | ⚡ | Performance |
| `style` | 💄 | Só formatação |

Se um commit mistura preocupações, escolha o tipo **principal** (em geral `feat` ou `fix`) e mencione docs no corpo. Prefira atualizar markdown **no mesmo commit** do comportamento.

## Exemplos (este repo)

```
✨ feat: add Lua execute subcommand on Rust CLI
📝 docs: add persistent CONTEXT for Cursor sessions
🙈 chore: ignore build artifacts and vector store
🤖 chore: add follow-spec agent skill
🔒 chore: lock Rust dependencies
```

## Nunca

- Commitar `target/`, `vector_store.json`, `.env`, segredos
- `--no-verify` / pular hooks, salvo se o usuário pedir
- Force-push em `main`
- Push sem o usuário pedir
- Amend de commit que já está no remoto (salvo pedido explícito)

## Modo um-arquivo-por-commit

Se o usuário disser “commit each file” / “um commit por arquivo”:

1. Adicionar **um caminho**
2. Commitar com a tabela acima
3. Repetir

Caso contrário: **um commit** para a mudança relacionada.

## Depois do commit

- `git status` deve estar limpo para os arquivos pretendidos
- Push **somente** se pedido (`git push -u origin HEAD` se preciso)
