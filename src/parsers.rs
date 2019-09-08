use std::num::{ParseFloatError, ParseIntError};

fn trim_before_equals(input: &str) -> String {
    input.split('=').collect::<Vec<_>>()[1].trim().to_owned()
}

pub fn temp(input: &str) -> Result<f64, ParseFloatError> {
    let parsable = trim_before_equals(input)
        .trim_end_matches("'C")
        .trim()
        .to_owned();

    let value = parsable.parse::<f64>()?;
    Ok(value)
}

pub fn throttled(input: &str) -> Result<isize, ParseIntError> {
    let parsable = trim_before_equals(input)
        .trim_start_matches("0x")
        .to_owned();

    let bit_pattern = isize::from_str_radix(&parsable, 16)?;

    Ok(bit_pattern)
}

pub fn volts(input: &str) -> Result<f64, ParseFloatError> {
    let parsable = trim_before_equals(input)
        .trim_end_matches("V")
        .trim()
        .to_owned();

    let value = parsable.parse::<f64>()?;
    Ok(value)
}

pub fn frequency(input: &str) -> Result<isize, ParseIntError> {
    let parsable = trim_before_equals(input);
    let value = parsable.parse::<isize>()?;
    Ok(value)
}

pub fn mem(input: &str) -> Result<isize, ParseIntError> {
    let parsable = trim_before_equals(input)
        .trim_end_matches("M")
        .trim()
        .to_owned();

    let value = parsable.parse::<isize>()?;
    Ok(value)
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
        assert_eq!(42.8f64, temp("temp=42.8'C").unwrap())
    }

    #[test]
    fn test_throttled() {
        let bit_pat_isize = throttled("throttled=0x50000").unwrap();
        assert_eq!(bit_pat_isize, 327680isize);
        assert_eq!(format!("{:b}", &bit_pat_isize), "1010000000000000000");
    }

    #[test]
    fn test_volts() {
        assert_eq!(1.20f64, volts("core:   volt=1.20V").unwrap())
    }

    #[test]
    fn test_frequency() {
        assert_eq!(
            700000000isize,
            frequency("arm:    frequency(45)=700000000").unwrap()
        )
    }

    #[test]
    fn test_mem() {
        assert_eq!(448isize, mem("arm=448M").unwrap())
    }
}
