use std::{
    collections::HashSet,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use glob::glob_with;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    bytes::complete::{take, take_till, take_until},
    character::{
        complete::{line_ending, multispace0, not_line_ending, one_of},
        is_alphanumeric,
    },
    multi::{many0, many0_count, many1},
    sequence::{delimited, preceded, terminated},
    IResult,
};
use regex::Regex;

use crate::{
    common::{discard_line_ending, number_around_spaces, p_decimal, ws},
    VoiceLine,
};

//TODO @k = carry over the next thing?
// k Ignore non text, can be in the middle of things
// @n = new line ofc
// @t = special character?
// @h portrait change
// @e ignore? thought it was like @k
// @m00@f idk
// @h whatever can be deleted
// @o on start line idk what it is, can probably remove
// DONE @rREADING@kana@
// v n k r most important for now
//

fn parse_header(text: &str) -> IResult<&str, u64> {
    // let (r, line) = terminated(take_until("\n"), tag("\n"))(text).unwrap();
    let (rest_file, line) = discard_line_ending(text)?;
    let (line, _) = tag("ZBSPAC-TRANSLATION ENCODING japanese COUNT")(line)?;
    let (_, count) = number_around_spaces(line)?;
    let count = count.parse::<u64>().unwrap();
    Ok((rest_file, count))
}

fn extract_seg_number(text: &str) -> IResult<&str, u64> {
    let (r, _) = take(4_usize)(text)?;
    let (_, n) = number_around_spaces(r)?;
    let n = n.parse::<u64>().unwrap();

    Ok((r, n))
}

fn parse_voice(text: &str) -> IResult<&str, &str> {
    let (r, _) = many0(preceded(
        alt((tag("@i"), tag("@m"), tag("@s"), tag("@o"))),
        take_until("@"),
    ))(text)?;
    let (_, voice) = delimited(
        tag("@v"),
        take_while1(|c: char| c.is_alphanumeric()),
        one_of("@ 「"),
    )(r)?;
    Ok((text, voice))
}

fn parse_text(text: &str) -> IResult<&str, &str> {
    let (r, _) = alt((take_until("「"), take_until("@m00@f20")))(text)?;
    let (_, line) = alt((
        delimited(tag("「"), take_while1(|c: char| c != '」'), tag("」")),
        preceded(tag("@m00@f20"), take_while1(|c: char| c != '\r')),
    ))(r)?;
    Ok((text, line))
}

#[derive(Debug)]
struct Segment {
    n: u64,
    contents: String,
    not_text: bool,
}

pub fn nexas(script_folder: &str) -> HashSet<VoiceLine> {
    // let paths = fs::read_dir(script_folder).unwrap();
    let regex_ruby = Regex::new(r"@r(.*?)@(.*?)@").unwrap();
    let regex_t = Regex::new(r"@t[0-9]*").unwrap();
    let regex_s = Regex::new(r"@s[0-9]*").unwrap();
    let regex_h = Regex::new(r"@h[0-9a-zA-Z_]*").unwrap();
    let regex_m = Regex::new(r"@m[0-9a-zA-Z_]*").unwrap();
    let path = format!("{}/**/script.txt", script_folder);
    let mut lines: HashSet<VoiceLine> = HashSet::with_capacity(50000);
    // let path = format!("{}/Replay/script.txt", script_folder);
    for entry in glob::glob(&path)
        .expect("Failed to read glob pattern")
        .flatten()
    {
        println!("{}", entry.display());
        // let path = PathBuf::from("Aozora/Scripts/act1006/script.txt");
        let f = File::open(entry.as_path()).unwrap();
        let r = BufReader::new(f);
        let s = utf16_reader::read_to_string(r);
        // println!("{s}",);

        let (mut r, count) = parse_header(&s).unwrap();
        let mut lined;
        let mut segments: Vec<Segment> = Vec::with_capacity(count as usize + 10);

        (r, _) = discard_line_ending(r).unwrap();
        while !r.is_empty() {
            // println!("->{r}",);
            (r, lined) = discard_line_ending(r).unwrap();
            let (rest_seg, seg_number) = extract_seg_number(lined).unwrap();
            let not_text = rest_seg.contains("NOT-TEXT");
            (r, lined) = discard_line_ending(r).unwrap();

            (r, _) = discard_line_ending(r).unwrap();
            (r, _) = discard_line_ending(r).unwrap();
            (r, _) = discard_line_ending(r).unwrap();
            segments.push(Segment {
                n: seg_number,
                contents: lined.to_string(),
                not_text,
            });
        }

        assert!(count as usize == segments.len());
        segments.retain(|s| !s.not_text);

        let mut worth_parsing: Vec<String> = Vec::with_capacity(segments.len());
        let mut saw_atk = false;
        let mut buffer = String::from("");
        for segment in segments {
            let contains_atk = segment.contents.contains("@k");
            let contains_atv = segment.contents.contains("@v");
            if saw_atk || contains_atv {
                buffer.push_str(&segment.contents);
                saw_atk = contains_atk;
            }
            // if contains_atv {
            //     buffer.push_str(segment.contents.as_str());
            // }
            if !contains_atk {
                // TODO push to lines, empty buffer
                if !buffer.is_empty() {
                    worth_parsing.push(buffer);
                }
                buffer = String::from("");
            }
        }

        for line in worth_parsing {
            let (_, voice) = parse_voice(&line).unwrap();
            let (_, text) = parse_text(&line).unwrap();
            let text = text.replace("@n", "");
            let text = text.replace("@k", "");

            let text = regex_ruby.replace_all(&text, "$1[$2]");
            let text = regex_t.replace_all(&text, "");
            let text = regex_s.replace_all(&text, "");
            let text = regex_m.replace_all(&text, "");
            let text = regex_h.replace_all(&text, "");
            lines.insert(VoiceLine(
                format!("{voice}.ogg"),
                String::from(""),
                text.to_string(),
            ));
        }
    }

    lines
}
