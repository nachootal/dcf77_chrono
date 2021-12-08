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
const MAX_DAY: u32 = 31;

/// Codes a given day [0..31] into DCF77 bit field
pub fn code_day(input: u32) -> Result<u64, Error> {
   let section = SectionInBitfield {data_bit_mask: DAY_BIT_MASK,
                                      data_position: DAY_POSITION,
                                      parity_mask: 0,
                                      max_data: MAX_DAY};
    code_dcf77(input, section)
}

/// Extracts the day out of a dcf77 bitfield
pub fn process_day(input: u64) -> Result<u32, Error> {
    println!("Process Day: 0x{:X} - 0x{:X} - 0x{:X}", input, DAY_MASK, DAY_POSITION);
   let section = SectionInBitfield {data_bit_mask: DAY_BIT_MASK,
                                      data_position: DAY_POSITION,
                                      parity_mask: 0,
                                      max_data: MAX_DAY};
    decode_dcf77(input, section)
}

/// Mask for the day_of_week [0..59) in the DCF77 bit field
const DAY_OF_WEEK_BIT_MASK: u64 = 0x7;

/// Position of the bits for the day_of_week in the DCF77 bit field
const DAY_OF_WEEK_POSITION: u8 = 15;

/// Mask for the day_of_week aligned in the DCF77 bit field
const DAY_OF_WEEK_MASK: u64 = DAY_OF_WEEK_BIT_MASK << DAY_OF_WEEK_POSITION;

/// Maximum value for the day_of_week in a DCF77 bit field
const MAX_DAY_OF_WEEK: u32 = 7;

/// Codes a given day of the week [0..7) into DCF77 bit field
pub fn code_day_of_the_week(input: u8) -> Result<u64, Error> {
    let section = SectionInBitfield {data_bit_mask: DAY_OF_WEEK_BIT_MASK,
                                       data_position: DAY_OF_WEEK_POSITION,
                                       parity_mask: 0,
                                       max_data: MAX_DAY_OF_WEEK};
    code_dcf77(u32::from(input), section)
}

/// Extracts the day_of_week out of a dcf77 bitfield
pub fn process_day_of_week(input: u64) -> Result<u32, Error> {
    let section = SectionInBitfield {data_bit_mask: DAY_OF_WEEK_BIT_MASK,
                                       data_position: DAY_OF_WEEK_POSITION,
                                       parity_mask: 0,
                                       max_data: MAX_DAY_OF_WEEK};
    decode_dcf77(input, section)
}

/// Mask for the month [0..59) in the DCF77 bit field
const MONTH_BIT_MASK: u64 = 0x1F;

/// Position of the bits for the month in the DCF77 bit field
const MONTH_POSITION: u8 = 10;

/// Mask for the month aligned in the DCF77 bit field
const MONTH_MASK: u64 = MONTH_BIT_MASK << MONTH_POSITION;

/// Maximum value for the month in a DCF77 bit field
const MAX_MONTH: u32 = 12;

/// Codes a given month [0..12] into DCF77 bit field
pub fn code_month(input: u32) -> Result<u64, Error> {
    let section = SectionInBitfield {data_bit_mask: MONTH_BIT_MASK,
                                     data_position: MONTH_POSITION,
                                     parity_mask: 0,
                                     max_data: MAX_MONTH};
    code_dcf77(input, section)
}

/// Extracts the month out of a dcf77 bitfield
pub fn process_month(input: u64) -> Result<u32, Error> {
    println!("Process Month: 0x{:X} - 0x{:X} - 0x{:X}", input, MONTH_MASK, MONTH_POSITION);
    let section = SectionInBitfield {data_bit_mask: MONTH_BIT_MASK,
                                     data_position: MONTH_POSITION,
                                     parity_mask: 0,
                                     max_data: MAX_MONTH};
    decode_dcf77(input, section)
}

/// Mask for the year [0..59) in the DCF77 bit field
const YEAR_BIT_MASK: u64 = 0xFF;

/// Position of the bits for the year in the DCF77 bit field
const YEAR_POSITION: u8 = 2;

/// Mask for the year aligned in the DCF77 bit field
const YEAR_MASK: u64 = YEAR_BIT_MASK << YEAR_POSITION;

/// Maximum value for the year in a DCF77 bit field
const MAX_YEAR: u32 = 165;

/// Position of the parity bit for the years in the DCF77 bit field
const PARITY_YEAR_BIT_MASK: u64 = 1 << 1;

/// Codes a given year [0..) into DCF77 bit field
pub fn code_year(input: i32) -> Result<u64, Error> {
    let section = SectionInBitfield {data_bit_mask: YEAR_BIT_MASK,
                                     data_position: YEAR_POSITION,
                                     parity_mask: PARITY_YEAR_BIT_MASK,
                                     max_data: MAX_YEAR};
    code_dcf77(input.try_into().unwrap(), section)
}

/// Extracts the year out of a dcf77 bitfield
pub fn process_year(input: u64) -> Result<u32, Error> {
    println!("Process Year: 0x{:X} - 0x{:X} - 0x{:X} - 0x{:X}", input, YEAR_MASK, YEAR_POSITION, PARITY_YEAR_BIT_MASK);
    let section = SectionInBitfield {data_bit_mask: YEAR_BIT_MASK,
                                     data_position: YEAR_POSITION,
                                     parity_mask: PARITY_YEAR_BIT_MASK,
                                     max_data: MAX_YEAR};
    decode_dcf77(input, section)
}

#[cfg(test)]
mod tests {
    use crate::to_dcf77;
    use crate::from_dcf77;
    use chrono::prelude::*;
    /*
    #[test]
    fn test_process_day_of_week() {
        for fake_input in 0..=0xff {
            match to_dcf77(0, 0, 0, fake_input, 0, 0) {
                Ok(coded_day_of_week) => {
                    match from_dcf77(coded_day_of_week) {
                        Ok(decoded_day_of_week) => {
                            assert!(Local.ymd(0, 0, u32::from(fake_input)) == decoded_day_of_week.date())
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
    */
    #[test]
    fn test_process_day() {
        for fake_input in 0..=0xff {
            match to_dcf77(Utc.ymd(10, 10, fake_input).and_hms(10, 10, 10)) {
                Ok(coded_date) => {
                    match from_dcf77(coded_date) {
                        Ok(decoded_date) => {
                            assert!(Utc.ymd(10, 10, fake_input) == decoded_date.date())
                        }
                        Err(error) => {
                            println!("Error on decoding the dcf77 {:?}", error.into_inner());
                        }
                    }
                }
                Err(error) => {
                    println!("Error on creating the dcf77 {:?}", error.into_inner());
                }
            }
        }
    }
    /*
    #[test]
    fn test_process_month() {
        for fake_input in 0..=0xf {
            match to_dcf77(Utc.ymd(10, fake_input, 10).and_hms(10, 10, 10)) {
                Ok(coded_date) => {
                    match from_dcf77(coded_date) {
                        Ok(decoded_date) => {
                            println!("Invented Date: {:?} - Decoded Date: {:?}", Utc.ymd(10, fake_input, 10), decoded_date.date());
                            assert!(Utc.ymd(10, fake_input, 10) == decoded_date.date())
                        }
                        Err(input) => {
                            println!("Error on decoding date: {:?}", input);
                        }
                    }
                }
                Err(coded_date) => {
                    println!("Error on coding date: {:?}", coded_date);
                }
            }
        }
    }
    */
    #[test]
    fn test_process_year() {
        for fake_input in 0..=0xff {
            match to_dcf77(Utc.ymd(i32::from(fake_input), 10, 10).and_hms(10, 10, 10)) {
                Ok(coded_date) => {
                    match from_dcf77(coded_date) {
                        Ok(decoded_date) => {
                            println!("Invented Date: {:?} - Decoded Date: {:?}", Utc.ymd(i32::from(fake_input), 10, 10), decoded_date.date());
                            assert!(Utc.ymd(i32::from(fake_input), 10, 10) == decoded_date.date())
                        }
                        Err(error) => {
                            println!("Error on decoding date: {:?}", error.into_inner());
                        }
                    }
                }
                Err(error) => {
                    println!("Error on coding date: {:?}", error.into_inner());
                }
            }
        }
    }
}
