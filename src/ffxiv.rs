// use clap::Parser;

use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap, HashSet,
    },
    fs::{read_to_string, File},
    io::BufReader,
    path::PathBuf,
    str::FromStr,
};

use clap::ValueEnum;
use csv::Reader;
use serde::Deserialize;

use crate::VoiceLine;

#[derive(Debug, Clone, ValueEnum)]
pub enum Version {
    // All,
    ARR,
    HS,
    SB,
    SHB,
    EW,
    //DT
}

impl Version {
    pub fn to_deck_name(&self) -> &'static str {
        match &self {
            Version::ARR => "Final Fantasy XIV::A Realm Reborn",
            Version::HS => "Final Fantasy XIV::Heavensward",
            Version::SB => "Final Fantasy XIV::Stormblood",
            Version::SHB => "Final Fantasy XIV::Shadowbringers",
            Version::EW => "Final Fantasy XIV::Endwalker",
        }
    }

    pub fn to_file_name(&self) -> &'static str {
        match &self {
            Version::ARR => "ffxivARR.apkg",
            Version::HS => "ffxivHS.apkg",
            Version::SB => "ffxivSB.apkg",
            Version::SHB => "ffxivShB.apkg",
            Version::EW => "ffxivEW.apkg",
        }
    }
    pub fn to_digit(&self) -> u8 {
        match &self {
            Version::ARR => 2,
            Version::HS => 3,
            Version::SB => 4,
            Version::SHB => 5,
            Version::EW => 6,
        }
    }
    pub fn to_cut_str(&self) -> &'static str {
        match &self {
            Version::ARR => "ffxiv",
            Version::HS => "ex1",
            Version::SB => "ex2",
            Version::SHB => "ex3",
            Version::EW => "ex4",
        }
    }
}

pub fn ffxiv(
    script_folder: &str,
    voices_folder: &str,
    xpac: Version,
) -> std::collections::HashSet<crate::VoiceLine> {
    let digit = xpac.to_digit();
    let cut_str = xpac.to_cut_str();
    let path_csv = format!("{}/cut_scene/0{digit}*/*.csv", script_folder);
    let path_mp3 = format!("{}/{cut_str}/sound/voicem/**/*.mp3", voices_folder);
    let mut lines: HashSet<VoiceLine> = HashSet::with_capacity(50000);
    // let path = format!("{}/Replay/script.txt", script_folder);
    let mut parsed_csv: HashMap<(String, String), String> = HashMap::with_capacity(15000);
    for entry in glob::glob(&path_csv)
        .expect("Failed to read glob pattern")
        .flatten()
    {
        let f = File::open(entry.as_path()).unwrap();
        let r = BufReader::new(f);
        let mut rdr = Reader::from_reader(r);
        for result in rdr.records() {
            let record = result.unwrap();
            if parsed_csv
                .insert(convert_voiceman(&record[1]), convert_string(&record[2]))
                .is_some()
            {
                eprintln!("Duplicate found, somehow?");
            }
        }
        // for line in s.lines() {
        //     // line.split(',')
        // }
        // println!("{}", s.lines().take(5).nth(3).unwrap());
    }
    // dbg!(parsed_csv);

    for entry in glob::glob(&path_mp3)
        .expect("Failed to read glob pattern")
        .flatten()
    {
        let filename = entry.file_name().unwrap();
        let mut s = filename.to_str().unwrap().split('_');
        println!("{s:?}",);
        let a = s.nth(2).unwrap().to_string();
        let b = s.next().unwrap().to_string().to_uppercase();
        // if !parsed_csv.contains(&((a, b))) {}
        match parsed_csv.entry((a.clone(), b.clone())) {
            Occupied(_) => {}
            Vacant(_) => {
                println!("{a}, {b} in mp3s but not found in csvs",);
            }
        }
        // for line in s.lines() {
        //     // line.split(',')
        // }
        // println!("{}", s.lines().take(5).nth(3).unwrap());
    }

    for (k, v) in parsed_csv.iter() {
        let p_str = format!(
            "{cut_str}/sound/voicem/voiceman_{}/vo_voiceman_{}_{}_m_ja.scd.mp3",
            k.0,
            k.0,
            k.1.to_lowercase()
        );
        let p_str_full = format!(
            "{}/{cut_str}/sound/voicem/voiceman_{}/vo_voiceman_{}_{}_m_ja.scd.mp3",
            voices_folder,
            k.0,
            k.0,
            k.1.to_lowercase()
        );
        let path = PathBuf::from_str(&p_str_full).unwrap();
        if path.exists() {
            lines.insert(VoiceLine(p_str, String::from(""), v.to_string()));
        } else {
            println!("{}, {}, {} found in csv but not files", k.0, k.1, *v);
        }
    }
    lines
}

fn convert_string(record: &str) -> String {
    //TODO replace \u{3}, br, etc
    let record = record.replace('\u{3}', "");
    let record = record.replace("<br>", "");
    let record = record.replace("<?0x48>", "");
    let record = record.replace("<?0x4a>", "");
    record.to_string()
}

fn convert_voiceman(voiceman: &str) -> (String, String) {
    let mut s = voiceman.split('_');
    let a = s.nth(2).unwrap().to_string();
    let b = s.next().unwrap().to_string();
    (a, b)
}
