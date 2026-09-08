use crate::rag::{query, DEFAULT_TOP_K};
use anyhow::{Context, Result};
use mlua::{Function, Lua, RegistryKey, Result as LuaResult, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

struct Skill {
    #[allow(dead_code)]
    description: String,
    func: RegistryKey,
}

type Registry = RefCell<HashMap<String, Skill>>;

pub fn create_lua() -> LuaResult<Lua> {
    let lua = Lua::new();
    lua.set_app_data(Registry::new(HashMap::new()));
    let globals = lua.globals();

    globals.set(
        "log",
        lua.create_function(|_, msg: String| {
            println!("{msg}");
            Ok(())
        })?,
    )?;

    globals.set(
        "register_skill",
        lua.create_function(
            |lua, (name, description, func): (String, String, Function)| {
                println!("registered skill: {name}");
                let key = lua.create_registry_value(func)?;
                let registry = lua.app_data_ref::<Registry>().expect("skill registry");
                registry.borrow_mut().insert(
                    name,
                    Skill {
                        description,
                        func: key,
                    },
                );
                Ok(())
            },
        )?,
    )?;

    globals.set(
        "rag_query",
        lua.create_function(|lua, question: String| {
            let hits = query(&question, DEFAULT_TOP_K).map_err(mlua::Error::external)?;
            let table = lua.create_table()?;
            for (i, hit) in hits.iter().enumerate() {
                let row = lua.create_table()?;
                row.set("text", hit.text.as_str())?;
                row.set("score", hit.score)?;
                row.set("source", hit.source.as_str())?;
                table.set(i + 1, row)?;
            }
            Ok(table)
        })?,
    )?;

    globals.set(
        "run_skill",
        lua.create_function(|lua, (name, args): (String, Value)| run_skill(lua, &name, args))?,
    )?;

    Ok(lua)
}

fn run_skill(lua: &Lua, name: &str, args: Value) -> mlua::Result<Value> {
    let func = {
        let registry = lua.app_data_ref::<Registry>().expect("skill registry");
        let borrow = registry.borrow();
        let skill = borrow.get(name).ok_or_else(|| {
            mlua::Error::external(format!("skill '{name}' is not registered"))
        })?;
        lua.registry_value::<Function>(&skill.func)?
    };
    let args = match args {
        Value::Nil => Value::Table(lua.create_table()?),
        other => other,
    };
    func.call::<Value>(args)
}

pub fn execute_file(path: &Path) -> Result<()> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    let lua = create_lua().map_err(|e| anyhow::anyhow!("{e}"))?;
    lua.load(&source)
        .set_name(path.to_string_lossy().as_ref())
        .exec()
        .map_err(|e| anyhow::anyhow!("lua {}: {e}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_log() {
        let lua = create_lua().unwrap();
        lua.load(
            r#"
            log("hello from Lua")
            register_skill("hello", "demo", function(args)
              log("hello skill")
            end)
            run_skill("hello", {})
            "#,
        )
        .exec()
        .unwrap();
    }

    #[test]
    fn execute_hello_plugin() {
        let plugin = Path::new(env!("CARGO_MANIFEST_DIR")).join("../lua/plugins/hello.lua");
        execute_file(&plugin).unwrap();
    }
}
