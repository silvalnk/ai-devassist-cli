mod lua_host;
mod prompt;
mod rag;
mod security;
mod store;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "devassist", about = "Polyglot AI-engineering lab (offline stubs)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Load and run a Lua plugin
    Execute { script: PathBuf },
    /// Index markdown or query the vector store
    Rag {
        #[command(subcommand)]
        cmd: RagCmd,
    },
    /// Retrieve chunks (and a stub grounded answer)
    Ask {
        question: Vec<String>,
        /// JSON-RPC 2.0 object on stdout (Ruby bridge)
        #[arg(long)]
        json: bool,
    },
    /// Prompt templates
    Prompt {
        #[command(subcommand)]
        cmd: PromptCmd,
    },
    /// Prompt-injection evals
    Security {
        #[command(subcommand)]
        cmd: SecurityCmd,
    },
}

#[derive(Subcommand)]
enum RagCmd {
    /// Chunk + stub-embed markdown under <dir>
    Index { dir: PathBuf },
    /// Search by cosine similarity
    Query {
        question: Vec<String>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum PromptCmd {
    /// Time `render()` on each `*.lua` template
    Benchmark { dir: PathBuf },
}

#[derive(Subcommand)]
enum SecurityCmd {
    /// Run sanitize/validate against payloads
    Audit {
        #[arg(long)]
        payloads: Option<PathBuf>,
    },
}

fn join_question(parts: &[String]) -> Result<String> {
    let q = parts.join(" ");
    if q.trim().is_empty() {
        bail!("question is empty");
    }
    Ok(q)
}

fn print_hits(question: &str, json: bool) -> Result<()> {
    let hits = rag::query(&question, rag::DEFAULT_TOP_K)?;
    if json {
        println!("{}", rag::jsonrpc_hits(&hits));
        return Ok(());
    }
    if hits.is_empty() {
        println!("Not found.");
        return Ok(());
    }
    println!("ask: {question}");
    for (i, hit) in hits.iter().enumerate() {
        println!(
            "--- #{} score={:.4} source={} ---",
            i + 1,
            hit.score,
            hit.source
        );
        println!("{}", hit.text);
    }
    println!("\n--- stub answer ---");
    println!("{}", rag::stub_answer(question, &hits));
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Execute { script } => {
            lua_host::execute_file(&script).with_context(|| format!("execute {}", script.display()))?;
        }
        Commands::Rag { cmd } => match cmd {
            RagCmd::Index { dir } => {
                let n = rag::index_dir(&dir)?;
                println!("indexed {n} chunks from {}", dir.display());
            }
            RagCmd::Query { question, json } => {
                print_hits(&join_question(&question)?, json)?;
            }
        },
        Commands::Ask { question, json } => {
            print_hits(&join_question(&question)?, json)?;
        }
        Commands::Prompt { cmd } => match cmd {
            PromptCmd::Benchmark { dir } => prompt::benchmark(&dir)?,
        },
        Commands::Security { cmd } => match cmd {
            SecurityCmd::Audit { payloads } => security::audit(payloads.as_deref())?,
        },
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn help_lists_cap1_subcommands() {
        let mut cmd = Cli::command();
        let help = cmd.render_long_help().to_string();
        for name in ["execute", "rag", "ask", "prompt", "security"] {
            assert!(help.contains(name), "missing {name} in:\n{help}");
        }
    }
}
