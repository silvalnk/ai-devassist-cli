use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Time `render(ctx)` on every `*.lua` file in `dir`.
pub fn benchmark(dir: &Path) -> Result<()> {
    if !dir.is_dir() {
        bail!("{} is not a directory", dir.display());
    }

    let mut files: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("read {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("lua"))
        .collect();
    files.sort();

    if files.is_empty() {
        bail!("no .lua templates in {}", dir.display());
    }

    println!("template                         micros");
    println!("-------------------------------- ------");

    let lua = mlua::Lua::new();
    for path in files {
        let source = fs::read_to_string(&path)
            .with_context(|| format!("read {}", path.display()))?;
        lua.load(&source)
            .set_name(path.to_string_lossy().as_ref())
            .exec()
            .map_err(|e| anyhow::anyhow!("{}: {e}", path.display()))?;

        let start = Instant::now();
        let rendered: String = lua
            .load(r#"return render({ question = "why Lua?", context = "lab" })"#)
            .eval()
            .map_err(|e| anyhow::anyhow!("{} render(): {e}", path.display()))?;
        let micros = start.elapsed().as_micros();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("?");
        println!("{name:<32} {micros}");
        let _ = rendered;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn times_a_template() {
        let dir = std::env::temp_dir().join(format!("devassist-prompt-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let mut f = fs::File::create(dir.join("demo.lua")).unwrap();
        writeln!(
            f,
            r#"function render(ctx) return "Q=" .. ctx.question end"#
        )
        .unwrap();
        benchmark(&dir).unwrap();
        let _ = fs::remove_dir_all(&dir);
    }
}
