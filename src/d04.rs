use regex::Regex;
use std::collections::HashMap;

use crate::errors::AppResult;

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"^#(?:\d|[a-f]){6}$").unwrap();
    static ref ID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
}

const INPUT: &'static str = include_str!("../data/passports.txt");

#[derive(Debug)]
struct Height {
    value: u32,
    unit: String,
}

#[derive(Debug)]
struct Passport {
    id: String,
    country_id: Option<String>,
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: Height,
    hair_color: String,
    eye_color: String,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        if !(self.birth_year >= 1920 && self.birth_year <= 2002) {
            return false;
        }

        if !(self.issue_year >= 2010 && self.issue_year <= 2020) {
            return false;
        }

        if !(self.expiration_year >= 2020 && self.expiration_year <= 2030) {
            return false;
        }

        if self.height.unit == "cm" && !(self.height.value >= 150 && self.height.value <= 193) {
            return false;
        }

        if self.height.unit == "in" && !(self.height.value >= 59 && self.height.value <= 76) {
            return false;
        }

        if !(EYE_COLORS.contains(&self.eye_color.as_str())) {
            return false;
        }

        if !(HAIR_COLOR_REGEX.is_match(&self.hair_color)) {
            return false;
        }

        if !(ID_REGEX.is_match(&self.id)) {
            return false;
        }

        true
    }
}

pub fn run() -> AppResult<()> {
    let passports = INPUT
        .split("\n\n")
        .filter_map(|line| {
            let passport_parts = line
                .split_whitespace()
                .map(|part| {
                    let property_parts = part.split(":").collect::<Vec<_>>();
                    (
                        *property_parts.get(0).expect("Malformed passport"),
                        *property_parts.get(1).expect("Malformed passport"),
                    )
                })
                .collect::<HashMap<_, _>>();

            let id = match passport_parts.get("pid") {
                Some(id) => id.to_string(),
                None => return None,
            };

            let country_id = passport_parts.get("cid").map(|id| id.to_string());

            let birth_year = match passport_parts.get("byr") {
                Some(id) => id.parse::<u32>().expect("Malformed birth year"),
                None => return None,
            };

            let issue_year = match passport_parts.get("iyr") {
                Some(id) => id.parse::<u32>().expect("Malformed issue year"),
                None => return None,
            };

            let expiration_year = match passport_parts.get("eyr") {
                Some(id) => id.parse::<u32>().expect("Malformed expiration year"),
                None => return None,
            };

            let height = match passport_parts.get("hgt") {
                Some(&id) => {
                    let captures = HEIGHT_REGEX.captures(id)?;
                    let value = captures
                        .get(1)?
                        .as_str()
                        .parse::<u32>()
                        .expect("Malformed height");
                    let unit = captures.get(2)?.as_str().to_string();

                    Height { value, unit }
                }
                None => return None,
            };

            let hair_color = match passport_parts.get("hcl") {
                Some(id) => id.to_string(),
                None => return None,
            };

            let eye_color = match passport_parts.get("ecl") {
                Some(id) => id.to_string(),
                None => return None,
            };

            let passport = Passport {
                id,
                country_id,
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
            };

            match passport.is_valid() {
                true => Some(passport),
                false => None,
            }
        })
        .collect::<Vec<_>>();

    println!("{:?}", passports.len());
    Ok(())
}
