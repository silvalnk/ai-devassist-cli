-- System prompt template (CAP-6)
function render(ctx)
  local q = (ctx and ctx.question) or ""
  local ctx_text = (ctx and ctx.context) or ""
  return table.concat({
    "You are DevAssist, an offline lab assistant.",
    "Answer only from the provided context. If missing, say Not found.",
    "Context:",
    ctx_text,
    "Question: " .. q,
  }, "\n")
end
