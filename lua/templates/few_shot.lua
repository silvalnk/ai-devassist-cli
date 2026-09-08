-- Few-shot template
function render(ctx)
  local q = (ctx and ctx.question) or ""
  return table.concat({
    "Examples:",
    "Q: what is RAG? A: retrieve snippets, then generate with them.",
    "Q: what is a skill? A: a named reusable capability.",
    "Now answer: " .. q,
  }, "\n")
end
