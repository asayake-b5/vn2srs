use std::{
    collections::HashSet,
    fs::{self, File},
    io::BufReader,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    combinator::rest,
    sequence::preceded,
    IResult,
};
use scraper::{Html, Selector};

use crate::VoiceLine;

fn parse_kgk(text: &str) -> IResult<&str, &str> {
    let (rest, name) = preceded(alt((tag("＠"), tag("@"))), take_until("\n"))(text)?;
    let (_, line) = take_till(|c| c == '@' || c == '/' || c == '＠')(rest)?;
    Ok((name, line))
}

fn ignore_comment(text: &str) -> IResult<&str, &str> {
    let (_, _) = preceded(tag("//"), rest)(text)?;
    Ok(("invalid", ""))
}

pub fn ken_ga_kimi(script_folder: &str) -> HashSet<VoiceLine> {
    let paths = fs::read_dir(script_folder).unwrap();
    let selector = Selector::parse("voice").unwrap();

    let mut lines: HashSet<VoiceLine> = HashSet::with_capacity(50000);

    for path in paths {
        // println!("{}", path.as_ref().unwrap().path().display());
        let f = File::open(path.as_ref().unwrap().path()).unwrap();
        let r = BufReader::new(f);
        let s = utf16_reader::read_to_string(r);
        let fragment = Html::parse_fragment(&s);
        for element in fragment.select(&selector) {
            // println!("-----",);
            let mut voice_file = element.value().attr("name").unwrap().to_string();
            voice_file.push_str(".mp3");
            let mut text = String::from("");
            for child in element.children() {
                let mut child2 = child;
                let mut breaked = false;
                while !child2.value().is_text() || breaked {
                    if let Some(c) = child2.first_child() {
                        child2 = c;
                        // println!("{}", c.value().as_element().unwrap().name());
                    } else {
                        breaked = true;
                    }
                    // child2 = child2.first_child().unwrap_(|| breaked = true);
                }
                if breaked {
                    break;
                }
                text.push_str(child2.value().as_text().unwrap());
                // println!("{}", child2.value().as_text().unwrap().to_string());

                // if child.value().is_text() {
                //     text.push_str(child.value().as_text().unwrap());
                //     println!("{}", child.value().as_text().unwrap().to_string());
                // } else {
                //     if child.value().as_element().unwrap().name() == "bgm" {
                //         for children in child.children() {
                //             if children.value().is_text() {
                //                 text.push_str(children.value().as_text().unwrap());
                //                 println!("{}", children.value().as_text().unwrap().to_string());
                //             }
                //         }
                //     }
                // }
            }
            // println!("{}", path.as_ref().unwrap().path().display());
            // println!("{voice_file}");
            let text = text.replace("%name%", "香夜").trim().to_string();
            // println!(":{text}");
            // println!("-",);
            let (speaker, line) = if voice_file.starts_with("NAR_") {
                ("narration", text.as_str())
            } else {
                alt((parse_kgk, ignore_comment))(&text).unwrap()
            };
            let line = line.trim().replace('\n', "");
            lines.insert(VoiceLine(voice_file, speaker.to_string(), line));
            // println!("{voice_file}:{text}")
            // println!("{:?}", element.children());
            // println!("{:?}", element.value().attr("name"));
            // println!("{}", element.text().collect::<String>());
            // println!("-----",);
        }
    }
    lines
}
