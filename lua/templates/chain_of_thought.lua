-- Chain-of-thought template (explicit steps, not hidden reasoning)
function render(ctx)
  local q = (ctx and ctx.question) or ""
  return table.concat({
    "Think in numbered steps, then answer.",
    "1. Restate the question.",
    "2. Use only retrieved context.",
    "3. Give a short conclusion.",
    "Question: " .. q,
  }, "\n")
end
