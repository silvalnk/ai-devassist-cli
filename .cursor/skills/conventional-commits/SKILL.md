---
name: conventional-commits
description: Cria commits git do DevAssist CLI no formato conventional commits + emoji, em inglês. Use quando o usuário pedir commit, conventional commit, mensagem de commit, git commit, ou um commit por arquivo.
---

# Conventional commits + emoji

Antes de commitar, leia [`.specify/COMMITS.md`](../../.specify/COMMITS.md) e siga à risca.

## Formato

```
<emoji> <type>: <subject>
```

**Inglês.** Subject no imperativo. Corpo opcional = **porquê**.

| Tipo | Emoji |
|------|-------|
| feat | ✨ |
| fix | 🐛 |
| docs | 📝 |
| chore (gitignore) | 🙈 |
| chore (lockfile) | 🔒 |
| chore (skills do Cursor) | 🤖 |
| chore (Cargo.toml / Gemfile) | 📦 |
| refactor | ♻️ |
| test | ✅ |

Padrão: **um commit** para a mudança relacionada (inclua o markdown correspondente nesse commit).

Se o usuário pedir **um commit por arquivo**: adicione e committe cada caminho separadamente usando a tabela.

## Segurança (este repo)

- Não committe `target/`, `vector_store.json`, `.env` ou segredos
- Não pule hooks
- Não faça push sem pedido
- Nunca altere git config
- HEREDOC para a mensagem (regras git do usuário)

Siga também o protocolo git global do usuário: `git status`, `git diff`, `git log` antes de commitar.
