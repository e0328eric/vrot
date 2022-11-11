use std::fs;
use std::io::{self, prelude::*};

use clap::Parser;
use rand::prelude::*;
use rustyline::{
    completion::FilenameCompleter,
    config::{CompletionType, Config},
    error::ReadlineError,
    Editor,
};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};
use serde::{Deserialize, Serialize};

const FILENAME_INPUT_PROMPT: &str = "Enter filenames: ";
const MAIN_PROMPT: &str = "Do you know this word? (q/y/N): ";
const INIT_BUFFER_CAPACITY: usize = 100 * 100;
const STDOUT_BUFFER_CAPACITY: usize = 200 * 100;

#[derive(Parser)]
#[command(author, version, about)]
struct VrotFlags {
    #[arg(long = "fuzzy")]
    is_fuzzy: bool,
}

#[derive(Debug)]
enum VrotErr {
    IOErr(std::io::Error),
    RustylineInitFailed,
    RustylineInternalErr,
    YamlParseFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Voca {
    word: String,
    info: Vec<VocaInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VocaInfo {
    meaning: String,
    synos: Vec<String>,
}

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
struct VrotHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
}

impl From<std::io::Error> for VrotErr {
    fn from(err: std::io::Error) -> Self {
        Self::IOErr(err)
    }
}

fn main() -> Result<(), VrotErr> {
    // Initial State of Vrot
    let cli = VrotFlags::parse();

    let config = Config::builder()
        .completion_type(if cli.is_fuzzy {
            CompletionType::Fuzzy
        } else {
            CompletionType::Circular
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
    let filename_input = take_file_rl.readline(FILENAME_INPUT_PROMPT);
    let mut buf = String::with_capacity(INIT_BUFFER_CAPACITY);
    match filename_input {
        Ok(filenames) => read_to_string_from_files(&mut buf, &filenames)?,
        Err(ReadlineError::Interrupted | ReadlineError::Eof) => return Ok(()),
        Err(err) => {
            eprintln!("ERROR: {err:?}");
            return Err(VrotErr::RustylineInternalErr);
        }
    }

    let vocas: Vec<Voca> = match serde_yaml::from_str(&buf) {
        Ok(vocas) => vocas,
        Err(err) => {
            eprintln!("{err:?}");
            return Err(VrotErr::YamlParseFailed);
        }
    };

    let mut rng = rand::thread_rng();
    let mut idx: usize;

    println!("\x1b[2J\x1b[H");
    let Ok(mut main_rl) = Editor::<()>::new() else {
        return Err(VrotErr::RustylineInitFailed);
    };
    loop {
        idx = rng.gen_range(0..vocas.len());
        display_voca_word(&mut stdout_buf, &vocas, idx)?;
        let readline = main_rl.readline(MAIN_PROMPT);
        println!("");
        match readline {
            Ok(val) => match val.as_str() {
                "q" | "quit" => break,
                "y" | "Y" => continue,
                _ => show_answer(&mut stdout_buf, &vocas, idx)?,
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

fn read_to_string_from_files(buf: &mut String, files: &str) -> Result<(), VrotErr> {
    let iter = files.split(char::is_whitespace);
    for filename in iter {
        let mut file = fs::File::open(filename)?;
        file.read_to_string(buf)?;
        buf.push('\n');
    }

    Ok(())
}

fn display_voca_word(
    stdout: &mut io::BufWriter<io::Stdout>,
    vocas: &[Voca],
    idx: usize,
) -> io::Result<()> {
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

fn show_answer(
    stdout: &mut io::BufWriter<io::Stdout>,
    vocas: &[Voca],
    idx: usize,
) -> io::Result<()> {
    let infos = &vocas[idx].info;
    for (i, info) in infos.iter().enumerate() {
        writeln!(stdout, "  Info {i}")?;
        writeln!(stdout, "  Meaning: {}", &info.meaning)?;
        writeln!(stdout, "  Synonyms: {}\n", join(&info.synos))?;
    }

    Ok(())
}

fn join(strings: &[String]) -> String {
    let mut output = String::with_capacity(strings.len() * 25);
    for s in strings {
        output += s;
        output += ", ";
    }
    output.pop();
    output.pop();

    output
}
