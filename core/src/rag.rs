use crate::store::{Entry, Hit, VectorStore, STORE_PATH};
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub const EMBED_DIM: usize = 128;
pub const DEFAULT_TOP_K: usize = 5;

pub fn embed(text: &str) -> Vec<f32> {
    let mut v = vec![0.0f32; EMBED_DIM];
    for token in tokenize(text) {
        let idx = hash_token(&token) % EMBED_DIM;
        v[idx] += 1.0;
    }
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut v {
            *x /= norm;
        }
    }
    v
}

fn tokenize(text: &str) -> Vec<String> {
    let folded: String = text
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();
    folded
        .split_whitespace()
        .filter(|t| t.len() >= 2 && !is_stopword(t))
        .map(|s| s.to_string())
        .collect()
}

fn is_stopword(token: &str) -> bool {
    matches!(
        token,
        "que" | "um" | "uma" | "de" | "do" | "da" | "dos" | "das" | "em" | "no" | "na"
            | "os" | "as" | "para" | "com" | "por" | "como" | "e" | "o" | "a" | "the"
            | "and" | "of" | "to" | "in"
    )
}

fn hash_token(token: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    hasher.finish() as usize
}

pub fn chunk_markdown(content: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut saw_heading = false;

    for line in content.lines() {
        if line.starts_with('#') {
            saw_heading = true;
            if !current.trim().is_empty() {
                chunks.push(current.trim().to_string());
                current.clear();
            }
        }
        current.push_str(line);
        current.push('\n');
    }
    if !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }

    if !saw_heading {
        chunks.clear();
        for para in content.split("\n\n") {
            let t = para.trim();
            if !t.is_empty() {
                chunks.push(t.to_string());
            }
        }
    }

    chunks.retain(|c| !c.trim().is_empty());
    chunks
}

fn collect_markdown(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("read {}", dir.display()))? {
        let path = entry?.path();
        if path.is_dir() {
            collect_markdown(&path, out)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

pub fn index_dir(dir: &Path) -> Result<usize> {
    if !dir.is_dir() {
        bail!("{} is not a directory", dir.display());
    }

    let mut files = Vec::new();
    collect_markdown(dir, &mut files)?;
    files.sort();

    let mut store = VectorStore::new();
    for path in files {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("read {}", path.display()))?;
        let source = path.to_string_lossy().replace('\\', "/");
        for chunk in chunk_markdown(&content) {
            if chunk.chars().filter(|c| !c.is_whitespace()).count() < 60 {
                continue;
            }
            store.insert(Entry {
                embedding: embed(&format!("{source}\n{chunk}")),
                text: chunk,
                source: source.clone(),
            });
        }
    }

    if store.entries.is_empty() {
        bail!("no markdown chunks in {}", dir.display());
    }

    store.save(Path::new(STORE_PATH))?;
    Ok(store.entries.len())
}

pub fn query(question: &str, k: usize) -> Result<Vec<Hit>> {
    let store = VectorStore::load(Path::new(STORE_PATH))?;
    if store.entries.is_empty() {
        bail!("vector store empty — run `rag index` first");
    }
    let ranked = store.rank_all(&embed(question));
    let mut related = crate::store::related_hits(ranked);
    if related.len() > k {
        related.truncate(k);
    }
    Ok(related)
}

pub fn stub_answer(question: &str, hits: &[Hit]) -> String {
    if hits.is_empty() {
        return "Not found.".into();
    }
    let mut out = format!(
        "Grounded stub (no paid LLM) for: {question}\n\nSources:\n"
    );
    for hit in hits {
        let snippet: String = hit.text.chars().take(180).collect();
        out.push_str(&format!("- {} — {snippet}…\n", hit.source));
    }
    out
}

#[derive(Serialize)]
struct JsonRpc<'a> {
    jsonrpc: &'static str,
    result: HitsBody<'a>,
    id: u32,
}

#[derive(Serialize)]
struct HitsBody<'a> {
    hits: &'a [Hit],
}

pub fn jsonrpc_hits(hits: &[Hit]) -> String {
    serde_json::to_string(&JsonRpc {
        jsonrpc: "2.0",
        result: HitsBody { hits },
        id: 1,
    })
    .expect("jsonrpc")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embed_is_deterministic() {
        let a = embed("why do we use Lua?");
        let b = embed("why do we use Lua?");
        assert_eq!(a, b);
        assert_eq!(a.len(), EMBED_DIM);
    }

    #[test]
    fn embed_same_tokens_rank_higher() {
        let q = embed("Lua plugins");
        let d = embed("# Lua\n\nLua is a lightweight plugin language.");
        let other = embed("security jailbreak payloads and sanitization");
        let sim = crate::store::cosine(&q, &d);
        let sim_other = crate::store::cosine(&q, &other);
        assert!(sim > sim_other);
    }

    #[test]
    fn chunk_splits_on_headings() {
        let text = "# One\n\nalpha\n\n# Two\n\nbeta\n";
        let chunks = chunk_markdown(text);
        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].contains("One"));
        assert!(chunks[1].contains("Two"));
    }
}
