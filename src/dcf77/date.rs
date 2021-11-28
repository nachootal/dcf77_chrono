use std::io::Error;
use super::*;

// As documented: https://www.cyber-sciences.com/wp-content/uploads/2019/01/TN-103_DCF77.pdf
/// Mask for the day [0..24) in the DCF77 bit field
const DAY_BIT_MASK: u64 = 0x3F;

/// Position of the bits for the day in the DCF77 bit field
const DAY_POSITION: u8 = 18;

/// Mask for the day aligned in the DCF77 bit field
const DAY_MASK: u64 = DAY_BIT_MASK << DAY_POSITION;

/// Maximum value for the day in a DCF77 bit field
const MAX_DAY: u8 = 31;

/// Mask for the day_of_week [0..59) in the DCF77 bit field
const DAY_OF_WEEK_BIT_MASK: u64 = 0x7;

/// Position of the bits for the day_of_week in the DCF77 bit field
const DAY_OF_WEEK_POSITION: u8 = 15;

/// Mask for the day_of_week aligned in the DCF77 bit field
const DAY_OF_WEEK_MASK: u64 = DAY_OF_WEEK_BIT_MASK << DAY_OF_WEEK_POSITION;

/// Maximum value for the day_of_week in a DCF77 bit field
const MAX_DAY_OF_WEEK: u8 = 7;

/// Mask for the month [0..59) in the DCF77 bit field
const MONTH_BIT_MASK: u64 = 0x7;

/// Position of the bits for the month in the DCF77 bit field
const MONTH_POSITION: u8 = 10;

/// Mask for the month aligned in the DCF77 bit field
const MONTH_MASK: u64 = DAY_OF_WEEK_BIT_MASK << DAY_OF_WEEK_POSITION;

/// Maximum value for the month in a DCF77 bit field
const MAX_MONTH: u8 = 12;

/// Mask for the year [0..59) in the DCF77 bit field
const YEAR_BIT_MASK: u64 = 0xFF;

/// Position of the bits for the year in the DCF77 bit field
const YEAR_POSITION: u8 = 2;

/// Mask for the year aligned in the DCF77 bit field
const YEAR_MASK: u64 = DAY_OF_WEEK_BIT_MASK << DAY_OF_WEEK_POSITION;

/// Maximum value for the year in a DCF77 bit field
const MAX_YEAR: u16 = 3000;

/// Codes a given day [0..31] into DCF77 bit field
pub fn code_day(input: u8) -> Result<u64, Error> {
    code_generic(input.into(), DAY_BIT_MASK, DAY_POSITION, 0, MAX_DAY.into())
}

/// Codes a given day of the week [0..7) into DCF77 bit field
pub fn code_day_of_the_week(input: u8) -> Result<u64, Error> {
    code_generic(input.into(), DAY_OF_WEEK_BIT_MASK, DAY_OF_WEEK_POSITION, 0, MAX_DAY_OF_WEEK.into())
}

/// Codes a given month [0..12] into DCF77 bit field
pub fn code_month(input: u8) -> Result<u64, Error> {
    code_generic(input.into(), MONTH_BIT_MASK, MONTH_POSITION, 0, MAX_MONTH.into())
}

/// Codes a given year [0..) into DCF77 bit field
pub fn code_year(input: u16) -> Result<u64, Error> {
    code_generic(input, YEAR_BIT_MASK, YEAR_POSITION, 0, MAX_YEAR)
}

/// Extracts the day out of a dcf77 bitfield
pub fn process_day(input: u64) -> u8 {
    decode_generic(input, DAY_MASK, DAY_POSITION) as u8
}

/// Extracts the day_of_week out of a dcf77 bitfield
pub fn process_day_of_week(input: u64) -> u8 {
    decode_generic(input, DAY_OF_WEEK_MASK, DAY_OF_WEEK_POSITION) as u8
}

/// Extracts the month out of a dcf77 bitfield
pub fn process_month(input: u64) -> u8 {
    decode_generic(input, MONTH_MASK, MONTH_POSITION) as u8
}

/// Extracts the year out of a dcf77 bitfield
pub fn process_year(input: u64) -> u16 {
    decode_generic(input, YEAR_MASK, YEAR_POSITION)
}

#[cfg(test)]
mod tests {
    use crate::to_dcf77;
    use crate::from_dcf77;
    #[test]
    fn test_process_day_of_week() {
        for fake_input in 0..=0xff {
            match to_dcf77(0, 0, 0, fake_input, 0, 0) {
                Ok(coded_day_of_week) => {
                    match from_dcf77(coded_day_of_week) {
                        Ok(decoded_day_of_week) => {
                            assert!(0 == decoded_day_of_week.day);
                            assert!(fake_input == decoded_day_of_week.day_of_week);
                            assert!(0 == decoded_day_of_week.month);
                            assert!(0 == decoded_day_of_week.year);
                        }
                        Err(input) => {
                            //assert!(MAX_DAY > input)
                            println!("Hello");
                        }
                    }
                }
                Err(coded_day) => {
                    //assert!(MAX_DAY < coded_day)
                    println!("Hello");
                }
            }
        }
    }
    #[test]
    fn test_process_day() {
        for fake_input in 0..=0xff {
            match to_dcf77(0, 0, fake_input, 0, 0, 0) {
                Ok(coded_day) => {
                    match from_dcf77(coded_day) {
                        Ok(decoded_day) => {
                            assert!(fake_input == decoded_day.day);
                            assert!(0 == decoded_day.day_of_week);
                            assert!(0 == decoded_day.month);
                            assert!(0 == decoded_day.year);
                        }
                        Err(input) => {
                            //assert!(MAX_DAY > input)
                            println!("Hello");
                        }
                    }
                }
                Err(coded_day) => {
                    //assert!(MAX_DAY < coded_day)
                    println!("Hello");
                }
            }
        }
    }
    #[test]
    fn test_process_month() {
        for fake_input in 0..=0xff {
            match to_dcf77(0, 0, 0, fake_input, 0, 0) {
                Ok(coded_day) => {
                    match from_dcf77(coded_day) {
                        Ok(decoded_day) => {
                            assert!(0 == decoded_day.day);
                            assert!(0 == decoded_day.day_of_week);
                            assert!(fake_input == decoded_day.month);
                            assert!(0 == decoded_day.year);
                        }
                        Err(input) => {
                            //assert!(MAX_DAY > input)
                            println!("Hello");
                        }
                    }
                }
                Err(coded_day) => {
                    //assert!(MAX_DAY < coded_day)
                    println!("Hello");
                }
            }
        }
    }
    #[test]
    fn test_process_year() {
        for fake_input in 0..=0xffff {
            match to_dcf77(0, 0, 0, 0, 0, fake_input) {
                Ok(coded_day) => {
                    match from_dcf77(coded_day) {
                        Ok(decoded_day) => {
                            assert!(0 == decoded_day.day);
                            assert!(0 == decoded_day.day_of_week);
                            assert!(0 == decoded_day.month);
                            assert!(fake_input == decoded_day.year);
                        }
                        Err(input) => {
                            //assert!(MAX_DAY > input)
                            println!("Hello");
                        }
                    }
                }
                Err(coded_day) => {
                    //assert!(MAX_DAY < coded_day)
                    println!("Hello");
                }
            }
        }
    }
}
