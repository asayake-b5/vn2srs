// fn parse_input(input: &str) -> IResult<&str, &str> {
//     take_until("voice")(input)
// }

// // TODO also manage escaped \"\" etc
// fn parse_line(input: &str) -> IResult<&str, &str> {
//     delimited(tag("\""), take_while(|c| c != '"'), tag("\""))(input)
// }

// let script = include_str!("../script.rpy");
// let (leftover_input, output) = parse_input(script)?;
// println!("{output}");

// let (i, o) = parse_line(
//     r#""「工房の件を使うか。孤児院のあまりの\"{rb}惨状{/rb}{rt}さんじょう{/rt}を{rb}憐{/rb}{rt}あわ{/rt}れんで、ローゼマインは孤児に仕事と食事を与えた。その{rb}献身{/rb}{rt}けんしん{/rt}ぶりと新しい事業が領主の目に留まったことにしよう」""#,
// )?;
// println!(".{i}.");
// println!("{o:?}");
