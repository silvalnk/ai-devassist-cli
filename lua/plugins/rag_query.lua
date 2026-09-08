-- Query the Rust vector store from Lua (CAP-3 host API)
log("rag_query plugin")
local question = "why do we use Lua?"
local hits = rag_query(question)
if #hits == 0 then
  log("Not found.")
else
  for i, hit in ipairs(hits) do
    log(string.format("#%d score=%.4f %s", i, hit.score, hit.source))
  end
end
