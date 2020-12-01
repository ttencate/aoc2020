use std::error::Error;
use std::fs;
use std::io;
use std::io::BufRead;

pub fn get_input(year: u32, day: u32) -> String {
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

fn input_file_name(_year: u32, day: u32) -> String {
    format!("input/{:02}.in", day)
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
    let client = reqwest::Client::new();
    let session_cookie = load_session_cookie()?;
    client.get(&url)
        .header(reqwest::header::COOKIE, format!("session={}", session_cookie))
        .send()
        .expect("request failed")
        .error_for_status()?
        .text()
        .map_err(From::from)
}

