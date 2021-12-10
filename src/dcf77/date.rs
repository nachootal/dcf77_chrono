use std::io::Error;
use super::*;

// As documented: https://www.cyber-sciences.com/wp-content/uploads/2019/01/TN-103_DCF77.pdf
/// Mask for the day [0..24) in the DCF77 bit field
const DAY_BIT_MASK: u64 = 0x3F;

/// Position of the bits for the day in the DCF77 bit field
const DAY_POSITION: u8 = 18;

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
    use crate::DCF77;
    use chrono::prelude::*;
    #[test]
    fn test_process_day_of_week() {
        for fake_input in 1..=7 {
            let test_date = DCF77 {
                date: Utc.ymd(21, 11, fake_input).and_hms(10, 10, 0),
                antenna: false,
                announce_daily_saving_time: false,
                daily_saving_time: false,
                standard_time: false,
                bit_leap_second: false
            };
            match to_dcf77(test_date) {
                Ok(coded_day_of_week) => {
                    match from_dcf77(coded_day_of_week) {
                        Ok(decoded_day_of_week) => {
                            assert!(test_date.date == decoded_day_of_week.date)
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
    #[test]
    fn test_process_day() {
        for fake_input in 1..=30 {
            let test_date = DCF77 {
                date: Utc.ymd(21, 11, fake_input).and_hms(10, 10, 0),
                antenna: false,
                announce_daily_saving_time: false,
                daily_saving_time: false,
                standard_time: false,
                bit_leap_second: false
            };
            match to_dcf77(test_date) {
                Ok(coded_date) => {
                    match from_dcf77(coded_date) {
                        Ok(decoded_date) => {
                            assert!(test_date.date == decoded_date.date)
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
    #[test]
    fn test_process_month() {
        for fake_input in 1..=12 {
            let test_date = DCF77 {
                date: Utc.ymd(21, fake_input, 12).and_hms(21, 10, 0),
                antenna: false,
                announce_daily_saving_time: false,
                daily_saving_time: false,
                standard_time: false,
                bit_leap_second: false
            };
            match to_dcf77(test_date) {
                Ok(coded_date) => {
                    match from_dcf77(coded_date) {
                        Ok(decoded_date) => {
                            assert!(test_date.date == decoded_date.date)
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
    #[test]
    fn test_process_year() {
        for fake_input in 0..=0xfff {
            let test_date = DCF77 {
                date: Utc.ymd(i32::from(fake_input), 10, 10).and_hms(21, 10, 0),
                antenna: false,
                announce_daily_saving_time: false,
                daily_saving_time: false,
                standard_time: false,
                bit_leap_second: false
            };
            match to_dcf77(test_date) {
                Ok(coded_date) => {
                    match from_dcf77(coded_date) {
                        Ok(decoded_date) => {
                            assert!(test_date.date == decoded_date.date)
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
