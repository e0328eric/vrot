mod error;

use std::fs;
use std::io::{self, prelude::*};
use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools;
use rand::prelude::*;
use rustyline::{
    completion::FilenameCompleter,
    config::{CompletionType, Config},
    error::ReadlineError,
    DefaultEditor, Editor,
};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};
use serde::{Deserialize, Serialize};

use error::VrotErr;

const FILENAME_INPUT_PROMPT: &str = "Enter filenames: ";
const MAIN_PROMPT: &str = "Do you know this word? (q/y/N): ";
const INIT_BUFFER_CAPACITY: usize = 100 * 100;
const STDOUT_BUFFER_CAPACITY: usize = 200 * 100;

#[derive(Parser)]
#[command(author, version, about)]
struct VrotFlags {
    #[arg(long = "cycle")]
    is_not_fuzzy: bool,
    filenames: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Voca {
    voca: Vec<Word>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Word {
    word: String,
    info: Vec<WordInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WordInfo {
    meaning: String,
    synos: Option<Vec<String>>,
    example: Option<String>,
}

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
struct VrotHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
}

pub fn cli_main() -> error::Result<()> {
    // Initial State of Vrot
    let cli = VrotFlags::parse();

    let config = Config::builder()
        .completion_type(if cli.is_not_fuzzy {
            CompletionType::Circular
        } else {
            CompletionType::Fuzzy
        })
        .build();
    let helper = VrotHelper {
        completer: FilenameCompleter::new(),
    };

    let Ok(mut take_file_rl) = Editor::with_config(config) else {
        return Err(VrotErr::RustylineInitFailed);
    };
    take_file_rl.set_helper(Some(helper));

    let mut stdout_buf = io::BufWriter::with_capacity(STDOUT_BUFFER_CAPACITY, io::stdout());

    // Taking files
    let filename_input = if cli.filenames.is_empty() {
        take_file_rl.readline(FILENAME_INPUT_PROMPT)
    } else {
        Ok(cli
            .filenames
            .iter()
            .map(|path_buf| path_buf.to_string_lossy())
            .join(" "))
    };
    let mut buf = String::with_capacity(INIT_BUFFER_CAPACITY);
    match filename_input {
        Ok(filenames) => read_to_string_from_files(&mut buf, &filenames)?,
        Err(ReadlineError::Interrupted | ReadlineError::Eof) => return Ok(()),
        Err(err) => {
            eprintln!("ERROR: {err:?}");
            return Err(VrotErr::RustylineInternalErr);
        }
    }

    let voca: Voca = match toml::from_str(&buf) {
        Ok(voca) => voca,
        Err(err) => {
            eprintln!("{err:?}");
            return Err(VrotErr::TomlParseFailed);
        }
    };

    let mut rng = rand::thread_rng();
    let mut idx: usize;

    println!("\x1b[2J\x1b[H");
    let Ok(mut main_rl) = DefaultEditor::new() else {
        return Err(VrotErr::RustylineInitFailed);
    };
    loop {
        idx = rng.gen_range(0..voca.voca.len());
        display_voca_word(&mut stdout_buf, &voca, idx)?;
        let readline = main_rl.readline(MAIN_PROMPT);
        println!("");
        match readline {
            Ok(val) => match val.as_str() {
                "q" | "quit" => break,
                "y" | "Y" => continue,
                _ => show_answer(&mut stdout_buf, &voca, idx)?,
            },
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("ERROR: {err:?}");
                return Err(VrotErr::RustylineInternalErr);
            }
        }
    }

    Ok(())
}

fn read_to_string_from_files(buf: &mut String, files: &str) -> error::Result<()> {
    let iter = files.split(char::is_whitespace);
    for filename in iter {
        let mut file = fs::File::open(filename)?;
        file.read_to_string(buf)?;
        buf.push('\n');
    }

    Ok(())
}

// TODO: Use crossterm, making this more general on usage.
fn display_voca_word(
    stdout: &mut io::BufWriter<io::Stdout>,
    voca: &Voca,
    idx: usize,
) -> io::Result<()> {
    let vocas = &voca.voca;
    let word = &vocas[idx].word;
    writeln!(
        stdout,
        "\x1b[1m--------------------------------------------------"
    )?;
    writeln!(stdout, "|{word:^48}|")?;
    writeln!(
        stdout,
        "--------------------------------------------------\x1b[0m"
    )?;
    stdout.flush()?;

    Ok(())
}

fn show_answer(stdout: &mut io::BufWriter<io::Stdout>, voca: &Voca, idx: usize) -> io::Result<()> {
    let vocas = &voca.voca;
    let infos = &vocas[idx].info;
    for (i, info) in infos.iter().enumerate() {
        writeln!(stdout, "  Info {i}")?;
        writeln!(stdout, "  Meaning: {}", &info.meaning)?;
        if let Some(ref synos) = info.synos {
            writeln!(stdout, "  Synonyms: {}", join_string(synos))?;
        }
        if let Some(ref example) = info.example {
            writeln!(stdout, "  Example: {example}")?;
        }
        writeln!(stdout)?;
    }
    stdout.flush()?;

    Ok(())
}

fn join_string(strings: &[String]) -> String {
    let mut output = String::with_capacity(strings.len() * 25);
    for s in strings {
        output += s;
        output += ", ";
    }
    output.pop();
    output.pop();

    output
}
