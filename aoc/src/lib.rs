use kuchiki::parse_html;
use kuchiki::traits::TendrilSink;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io::BufRead;
use std::io;
use std::time::Instant;

pub fn main<P1, P2, R1, R2>(part1: P1, part2: P2)
    where P1: Fn(&str) -> R1, P2: Fn(&str) -> R2, R1: Display, R2: Display
{
    let year = year();
    let day = day();
    let input = input();
    run(year, day, 1, part1, &input);
    run(year, day, 2, part2, &input);
}

pub fn input() -> String {
    let year = year();
    let day = day();
    let input_file_name = input_file_name(year, day);
    fs::read_to_string(&input_file_name)
        .or_else(|_err| -> Result<String, Box<dyn Error>> {
            println!("Input file {} could not be read, fetching...", input_file_name);
            let contents = fetch_input(year, day)?;
            fs::write(&input_file_name, &contents)?;
            Ok(contents)
        })
        .unwrap()
}

pub fn example(index: usize) -> String {
    let year = year();
    let day = day();
    let example_file_name = example_file_name(year, day, index);
    fs::read_to_string(&example_file_name)
        .or_else(|_err| -> Result<String, Box<dyn Error>> {
            println!("Example file {} could not be read, fetching...", example_file_name);
            let num_examples = fetch_examples(year, day)?;
            if index >= num_examples {
                panic!(
                    "tried to read example {} but there are only {} examples for year {}, day {}",
                    index, num_examples, year, day);
            }
            let contents = fs::read_to_string(&example_file_name)
                .unwrap();
            Ok(contents)
        })
        .unwrap()
}

fn year() -> u32 {
    2020
}

fn day() -> u32 {
    env::current_exe().unwrap()
        .file_stem().unwrap()
        .to_str().unwrap()
        .get(0..2).unwrap()
        .parse::<u32>().unwrap()
}

fn run<P, R>(year: u32, day: u32, part: u32, func: P, input: &str)
    where P: Fn(&str) -> R, R: Display
{
    let start = Instant::now();
    let output = func(input);
    let duration = start.elapsed();

    println!(
        "Answer to {} day {}, part {} ({}.{:03} s): {}",
        year, day, part, duration.as_secs(), duration.subsec_millis(), output);
}

fn input_file_name(_year: u32, day: u32) -> String {
    format!("inputs/{:02}.in", day)
}

fn example_file_name(_year: u32, day: u32, index: usize) -> String {
    format!("examples/{:02}-{}.example", day, index)
}

fn load_session_cookie() -> Result<String, io::Error> {
    let cookie_file_name = ".session_cookie";
    fs::read_to_string(cookie_file_name)
        .map(|s| s.trim().to_string())
        .or_else(|_err| -> Result<String, io::Error> {
            println!("No session cookie found. Please log in to https://adventofcode.com/ in your browser, open the browser console, copy the value of the 'session' cookie, and paste it here:");
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line)?;
            fs::write(&cookie_file_name, &line)?;
            Ok(line.trim().to_string())
        })
}

fn fetch_input(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    send_get_request(&url)?
        .text()
        .map_err(From::from)
}

fn fetch_examples(year: u32, day: u32) -> Result<usize, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let mut response = send_get_request(&url)?;
    let root = parse_html().from_utf8().read_from(&mut response)?;
    let mut num_examples = 0;
    for (index, node) in root.select("pre").unwrap().enumerate() {
        fs::write(&example_file_name(year, day, index), node.text_contents())?;
        num_examples += 1;
    }
    Ok(num_examples)
}

fn send_get_request(url: &str) -> Result<reqwest::Response, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let session_cookie = load_session_cookie()?;
    let response = client.get(url)
        .header(reqwest::header::COOKIE, format!("session={}", session_cookie))
        .send()
        .expect("request failed")
        .error_for_status()?;
    Ok(response)
}
