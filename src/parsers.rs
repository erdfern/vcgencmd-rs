use std::string::ParseError;

fn trim_before_equals(input: &str) -> String {
    input.split("=").collect::<Vec<_>>()[1].to_owned()
}

pub fn temp(input: &str) -> String {
    let parsed = trim_before_equals(input).trim_end_matches("'C").to_owned();

    parsed
}

pub fn volts(input: &str) -> String {
    let parsed = trim_before_equals(input).trim_end_matches("V").to_owned();

    parsed
}

pub fn frequency(input: &str) -> String {
    trim_before_equals(input)
}

pub fn mem(input: &str) -> String {
    let parsed = trim_before_equals(input).trim_end_matches("M").to_owned();

    parsed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trim_before_equals() {
        assert_eq!("42.8'C", trim_before_equals("temp=42.8'C"));
        assert_eq!(
            "250000000",
            trim_before_equals("core:   frequency(1)=250000000")
        )
    }

    #[test]
    fn test_temp() {
        assert_eq!("42.8", temp("temp=42.8'C"))
    }

    #[test]
    fn test_volts() {
        assert_eq!("1.20", volts("core:   volt=1.20V"))
    }

    #[test]
    fn test_frequency() {
        assert_eq!("700000000", frequency("arm:    frequency(45)=700000000"))
    }

    #[test]
    fn test_mem() {
        assert_eq!("448", mem("arm=448M"))
    }
}
