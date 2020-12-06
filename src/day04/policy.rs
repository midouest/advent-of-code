use lazy_static::lazy_static;
use regex::Regex;

use super::passport::Passport;

pub trait ValidationPolicy {
    fn is_valid(passport: &Passport) -> bool;
}

pub struct RelaxedPolicy {}

impl ValidationPolicy for RelaxedPolicy {
    fn is_valid(passport: &Passport) -> bool {
        passport.birth_year.is_some()
            && passport.issue_year.is_some()
            && passport.expiration_year.is_some()
            && passport.height.is_some()
            && passport.hair_color.is_some()
            && passport.eye_color.is_some()
            && passport.passport_id.is_some()
    }
}

pub struct StrictPolicy {}

impl StrictPolicy {
    fn is_valid_year(year: &Option<String>, min: i64, max: i64) -> bool {
        year.as_ref()
            .and_then(|s| s.parse::<i64>().ok())
            .map(|i| i >= min && i <= max)
            .unwrap_or(false)
    }

    pub fn is_valid_birth_year(birth_year: &Option<String>) -> bool {
        Self::is_valid_year(birth_year, 1920, 2002)
    }

    pub fn is_valid_issue_year(birth_year: &Option<String>) -> bool {
        Self::is_valid_year(birth_year, 2010, 2020)
    }

    pub fn is_valid_expiration_year(birth_year: &Option<String>) -> bool {
        Self::is_valid_year(birth_year, 2020, 2030)
    }

    pub fn is_valid_height(height: &Option<String>) -> bool {
        height
            .as_ref()
            .and_then(|s| {
                s.find("cm")
                    .or_else(|| s.find("in"))
                    .map(|index| s.split_at(index))
            })
            .and_then(|(height, unit)| height.parse::<i64>().ok().map(|height| (height, unit)))
            .map(|(height, unit)| match unit {
                "cm" => height >= 150 && height <= 193,
                "in" => height >= 59 && height <= 76,
                _ => false,
            })
            .unwrap_or(false)
    }

    pub fn is_valid_hair_color(hair_color: &Option<String>) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        }
        hair_color.as_ref().map(|s| RE.is_match(s)).unwrap_or(false)
    }

    pub fn is_valid_eye_color(eye_color: &Option<String>) -> bool {
        eye_color
            .as_ref()
            .map(|s| {
                s == "amb"
                    || s == "blu"
                    || s == "brn"
                    || s == "gry"
                    || s == "grn"
                    || s == "hzl"
                    || s == "oth"
            })
            .unwrap_or(false)
    }

    pub fn is_valid_passport_id(passport_id: &Option<String>) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        passport_id
            .as_ref()
            .map(|s| RE.is_match(s))
            .unwrap_or(false)
    }
}

impl ValidationPolicy for StrictPolicy {
    fn is_valid(passport: &Passport) -> bool {
        Self::is_valid_birth_year(&passport.birth_year)
            && Self::is_valid_issue_year(&passport.issue_year)
            && Self::is_valid_expiration_year(&passport.expiration_year)
            && Self::is_valid_height(&passport.height)
            && Self::is_valid_hair_color(&passport.hair_color)
            && Self::is_valid_eye_color(&passport.eye_color)
            && Self::is_valid_passport_id(&passport.passport_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::day04::passport::Passport;

    use super::StrictPolicy;

    #[test]
    fn it_validates_partial_examples() {
        assert_eq!(
            StrictPolicy::is_valid_birth_year(&Some("2002".to_string())),
            true
        );

        assert_eq!(
            StrictPolicy::is_valid_birth_year(&Some("2003".to_string())),
            false
        );

        assert_eq!(
            StrictPolicy::is_valid_height(&Some("60in".to_string())),
            true
        );

        assert_eq!(
            StrictPolicy::is_valid_height(&Some("190cm".to_string())),
            true
        );

        assert_eq!(
            StrictPolicy::is_valid_height(&Some("190in".to_string())),
            false
        );

        assert_eq!(
            StrictPolicy::is_valid_height(&Some("190".to_string())),
            false
        );

        assert_eq!(
            StrictPolicy::is_valid_hair_color(&Some("#123abc".to_string())),
            true
        );

        assert_eq!(
            StrictPolicy::is_valid_hair_color(&Some("#123abz".to_string())),
            false
        );

        assert_eq!(
            StrictPolicy::is_valid_hair_color(&Some("123abc".to_string())),
            false
        );

        assert_eq!(
            StrictPolicy::is_valid_eye_color(&Some("brn".to_string())),
            true
        );

        assert_eq!(
            StrictPolicy::is_valid_eye_color(&Some("wat".to_string())),
            false
        );

        assert_eq!(
            StrictPolicy::is_valid_passport_id(&Some("000000001".to_string())),
            true
        );

        assert_eq!(
            StrictPolicy::is_valid_eye_color(&Some("0123456789".to_string())),
            false
        );
    }

    #[test]
    fn it_invalidates_passports() {
        let batch = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let passports = Passport::parse_batch(batch);
        let invalidated = passports
            .iter()
            .map(|p| p.is_valid::<StrictPolicy>())
            .all(|v| !v);
        assert!(invalidated);
    }

    #[test]
    fn it_validates_passports() {
        let batch = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passports = Passport::parse_batch(batch);
        let validated = passports
            .iter()
            .map(|p| p.is_valid::<StrictPolicy>())
            .all(|v| v);
        assert!(validated);
    }
}
