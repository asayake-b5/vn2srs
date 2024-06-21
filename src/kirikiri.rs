use std::{collections::HashSet, fs::read_to_string};

use serde::{Deserialize, Serialize};

use crate::VoiceLine;

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    speaker: String,
    line: String,
    voice: String,
}

pub fn kirikiri(script_folder: &str) -> HashSet<VoiceLine> {
    let mut lines: HashSet<VoiceLine> = HashSet::with_capacity(50000);
    let path = format!("{script_folder}/all.json");
    let json = read_to_string(&path).unwrap();
    let r: Vec<Vec<Entry>> = serde_json::from_str(&json).unwrap();
    let r = r.into_iter().flatten().collect::<Vec<Entry>>();
    dbg!(r.len());
    for entry in r {
        lines.insert(VoiceLine(
            format!("{}.ogg", entry.voice),
            entry.speaker,
            entry.line,
        ));
    }
    lines
}
