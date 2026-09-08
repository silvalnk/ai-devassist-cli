-- CAP-2: plugin demo — log + register_skill
log("hello from Lua")

register_skill("hello", "prints a greeting from a Lua plugin", function(args)
  log("hello skill ran")
end)

run_skill("hello", {})
