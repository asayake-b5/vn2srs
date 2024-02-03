use std::{
    collections::HashSet,
    error::Error,
    fs::{self},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use clap::Subcommand;
use genanki_rs::{Deck, Field, Model, Note, Package, Template};
use kengakimi::ken_ga_kimi;

pub mod common;
pub mod kengakimi;
pub mod nexas;
pub mod renpy;

const SILENCE_BYTES: &[u8] = include_bytes!("../silence.mp3");

#[derive(Debug, Eq, Hash, PartialEq)]
/// Voice File, Speaker, Line
pub struct VoiceLine(String, String, String);

pub fn anki_sentence(speaker: &str, line: &str) -> String {
    format!("{} 「{}」", speaker, line)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Visual novel to parse
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Ken Ga Kimi
    KenGaKimi {
        ///Folder containing the extracted script files (*_*_*.bytes etc etc)
        script_folder: String,
        ///Folder containing the mp3 voice files
        voices_folder: String,
    },
    /// NeXAs engine games (GIGA notably)
    Nexas {
        ///Folder containing the extracted script files (a/script.txt b/script.txt etc etc)
        script_folder: String,
        ///Folder containing the voic files (*.ogg)
        voices_folder: String,
        ///Game Name (eg "Baldr Sky")
        game_name: String,
        ///Output deck (eg "baldrsky.apkg")
        output_file: String,
    },
}
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let (deck_name, deck_file, _script_folder, voices_folder, lines) = match &cli.command {
        Commands::KenGaKimi {
            script_folder,
            voices_folder,
        } => (
            "剣が君",
            "kengakimi.apkg",
            script_folder,
            voices_folder,
            ken_ga_kimi(script_folder),
        ),
        Commands::Nexas {
            script_folder,
            voices_folder,
            game_name,
            output_file,
        } => (
            game_name.as_str(),
            output_file.as_str(),
            script_folder,
            voices_folder,
            nexas::nexas(script_folder),
        ),
    };

    let model = Model::new(
        1706559908,
        "vn2srs",
        vec![
            Field::new("Audio"),
            Field::new("Image"),
            Field::new("Sentence"),
            Field::new("SentenceNoCharacter"),
        ],
        vec![Template::new("Card 1")
            .qfmt("{{Sentence}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Audio}} {Image}"#)],
    );

    let mut files = HashSet::new();
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut deck = Deck::new(
        timestamp as i64,
        deck_name,
        &format!("{deck_name} - Generated by https://github.com/asayake-b5/vn2srs"),
    );
    let prefix = deck_name.replace(' ', "_");
    for line in lines {
        if line.1 == "invalid" {
            continue;
        }
        let original = format!("./{}/{}", voices_folder, line.0);
        let copy = format!("./{}/{}_{}", voices_folder, prefix, line.0);
        let in_path = Path::new(&original);
        let out_path = Path::new(&copy);
        if !in_path.exists() {
            println!("{}", in_path.display());
            println!("noexist",);
            fs::write(in_path, SILENCE_BYTES).unwrap();
        }
        fs::copy(in_path, out_path).unwrap();
        files.insert(copy);
        deck.add_note(
            Note::new(
                model.clone(),
                vec![
                    &format!("[sound:{}_{}]", prefix, line.0,),
                    "",
                    &anki_sentence(&line.1, &line.2),
                    &line.2,
                ],
            )
            .unwrap(),
        );
    }

    let files2: Vec<&str> = files.iter().map(|s| &**s).collect();
    let mut package = Package::new(vec![deck], files2)?;
    package.write_to_file(deck_file).unwrap();

    Ok(())
}
