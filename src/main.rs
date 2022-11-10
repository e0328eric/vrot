use std::fs;
use std::io::{self, prelude::*};

use rand::prelude::*;
use rustyline::{error::ReadlineError, Editor};
use serde::{Deserialize, Serialize};

const FILENAME_INPUT_PROMPT: &str = "Enter filenames: ";
const MAIN_PROMPT: &str = "Do you know this word? (q/y/N): ";
const INIT_BUFFER_CAPACITY: usize = 100 * 100;
const STDOUT_BUFFER_CAPACITY: usize = 200 * 100;

#[derive(Debug)]
enum VrotErr {
    IOErr(std::io::Error),
    RustylineInitFailed,
    RustylineInternalErr,
    JsonParseFailed,
    FileNotFound,
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

impl From<std::io::Error> for VrotErr {
    fn from(err: std::io::Error) -> Self {
        Self::IOErr(err)
    }
}

fn main() -> Result<(), VrotErr> {
    let Ok(mut rl) = Editor::<()>::new() else {
        return Err(VrotErr::RustylineInitFailed);
    };

    let mut stdout_buf = io::BufWriter::with_capacity(STDOUT_BUFFER_CAPACITY, io::stdout());

    let filename_input = rl.readline(FILENAME_INPUT_PROMPT);
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
            return Err(VrotErr::JsonParseFailed);
        }
    };

    let mut rng = rand::thread_rng();
    let mut idx: usize;

    println!("\x1b[2J\x1b[H");
    loop {
        idx = rng.gen_range(0..vocas.len());
        display_voca_word(&mut stdout_buf, &vocas, idx)?;
        let readline = rl.readline(MAIN_PROMPT);
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
