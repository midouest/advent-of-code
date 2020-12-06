use lazy_static::lazy_static;
use regex::{Captures, Regex};
use thiserror::Error;

use super::policy::ValidationPolicy;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Passport {
    pub birth_year: Option<String>,
    pub issue_year: Option<String>,
    pub expiration_year: Option<String>,
    pub height: Option<String>,
    pub hair_color: Option<String>,
    pub eye_color: Option<String>,
    pub passport_id: Option<String>,
    pub country_id: Option<String>,
}

impl Passport {
    pub fn new() -> Self {
        Self {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    pub fn with_birth_year(self, byr: &str) -> Self {
        Self {
            birth_year: Some(byr.to_string()),
            ..self
        }
    }

    pub fn with_issue_year(self, iyr: &str) -> Self {
        Self {
            issue_year: Some(iyr.to_string()),
            ..self
        }
    }

    pub fn with_expiration_year(self, eyr: &str) -> Self {
        Self {
            expiration_year: Some(eyr.to_string()),
            ..self
        }
    }

    pub fn with_height(self, hgt: &str) -> Self {
        Self {
            height: Some(hgt.to_string()),
            ..self
        }
    }

    pub fn with_hair_color(self, hcl: &str) -> Self {
        Self {
            hair_color: Some(hcl.to_string()),
            ..self
        }
    }

    pub fn with_eye_color(self, ecl: &str) -> Self {
        Self {
            eye_color: Some(ecl.to_string()),
            ..self
        }
    }

    pub fn with_passport_id(self, pid: &str) -> Self {
        Self {
            passport_id: Some(pid.to_string()),
            ..self
        }
    }

    pub fn with_country_id(self, cid: &str) -> Self {
        Self {
            country_id: Some(cid.to_string()),
            ..self
        }
    }

    pub fn parse_batch(batch: &str) -> Vec<Passport> {
        let lines = batch.lines();
        let mut buffer = Vec::new();
        let mut passports = Vec::new();

        for line in lines {
            if !line.is_empty() {
                buffer.push(line);
            } else {
                let passport = Passport::parse_lines(buffer);
                passports.push(passport);
                buffer = Vec::new();
            }
        }

        let passport = Passport::parse_lines(buffer);
        passports.push(passport);

        passports
    }

    fn parse_lines(lines: Vec<&str>) -> Passport {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]{3}):([a-z0-9#]+)").unwrap();
        }

        lines
            .into_iter()
            .flat_map(|line| RE.captures_iter(line))
            .fold(Passport::new(), |p, c: Captures| match (&c[1], &c[2]) {
                ("byr", byr) => p.with_birth_year(byr),
                ("iyr", iyr) => p.with_issue_year(iyr),
                ("eyr", eyr) => p.with_expiration_year(eyr),
                ("hgt", hgt) => p.with_height(hgt),
                ("hcl", hcl) => p.with_hair_color(hcl),
                ("ecl", ecl) => p.with_eye_color(ecl),
                ("pid", pid) => p.with_passport_id(pid),
                ("cid", cid) => p.with_country_id(cid),
                _ => p,
            })
    }

    pub fn is_valid<P>(&self) -> bool
    where
        P: ValidationPolicy,
    {
        P::is_valid(self)
    }
}

#[derive(Debug, Error)]
pub enum PassportError {
    #[error("Found unknown field '{0}:{1}' while parsing passport")]
    UnknownField(String, String),
}

pub type Result<T> = std::result::Result<T, PassportError>;

#[cfg(test)]
mod tests {
    use crate::day04::policy::RelaxedPolicy;

    use super::Passport;

    const BATCH: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn it_parses_a_batch_of_passports() {
        let passports = Passport::parse_batch(BATCH);
        assert_eq!(
            passports,
            vec![
                Passport::new()
                    .with_eye_color("gry")
                    .with_passport_id("860033327")
                    .with_expiration_year("2020")
                    .with_hair_color("#fffffd")
                    .with_birth_year("1937")
                    .with_issue_year("2017")
                    .with_country_id("147")
                    .with_height("183cm"),
                Passport::new()
                    .with_issue_year("2013")
                    .with_eye_color("amb")
                    .with_country_id("350")
                    .with_expiration_year("2023")
                    .with_passport_id("028048884")
                    .with_hair_color("#cfa07d")
                    .with_birth_year("1929"),
                Passport::new()
                    .with_hair_color("#ae17e1")
                    .with_issue_year("2013")
                    .with_expiration_year("2024")
                    .with_eye_color("brn")
                    .with_passport_id("760753108")
                    .with_birth_year("1931")
                    .with_height("179cm"),
                Passport::new()
                    .with_hair_color("#cfa07d")
                    .with_expiration_year("2025")
                    .with_passport_id("166559648")
                    .with_issue_year("2011")
                    .with_eye_color("brn")
                    .with_height("59in")
            ]
        );
    }

    #[test]
    fn it_validates_passports() {
        let valid: Vec<_> = Passport::parse_batch(BATCH)
            .into_iter()
            .map(|p| p.is_valid::<RelaxedPolicy>())
            .collect();

        assert_eq!(valid, vec![true, false, true, false]);
    }
}
