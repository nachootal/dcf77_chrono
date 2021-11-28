use std::io::Error;
use super::*;

// As documented: https://www.cyber-sciences.com/wp-content/uploads/2019/01/TN-103_DCF77.pdf
/// Mask for the hour [0..24) in the DCF77 bit field
const HOUR_BIT_MASK: u64 = 0x7E;

/// Position of the parity bit for the hour in the DCF77 bit field
const PARITY_HOUR_BIT_MASK: u64 = 1 << 24;

/// Position of the bits for the hour in the DCF77 bit field
const HOUR_POSITION: u8 = 25;

/// Mask for the hour aligned in the DCF77 bit field
const HOUR_MASK: u64 = HOUR_BIT_MASK << HOUR_POSITION;

/// Maximum value for the hour in a DCF77 bit field
const MAX_HOUR: u8 = 24;

/// Mask for the minutes [0..59) in the DCF77 bit field
const MINUTES_BIT_MASK: u64 = 0xFE;

/// Position of the parity bit for the minutes in the DCF77 bit field
const PARITY_MINUTES_BIT_MASK: u64 = 1 << 31;

/// Position of the bits for the minutes in the DCF77 bit field
const MINUTES_POSITION: u8 = 32;

/// Mask for the minutes aligned in the DCF77 bit field
const MINUTES_MASK: u64 = MINUTES_BIT_MASK << MINUTES_POSITION;

/// Maximum value for the minutes in a DCF77 bit field
const MAX_MINUTES: u8 = 60;

/// Codes a given hour [0..24) into DCF77 bit field
pub fn code_hour(input: u8) -> Result<u64, Error> {
    code_generic(input.into(), HOUR_BIT_MASK, HOUR_POSITION, PARITY_HOUR_BIT_MASK, MAX_HOUR.into())
}

/// Codes a given minutes [0..59) into DCF77 bit field
pub fn code_minutes(input: u8) -> Result<u64, Error> {
    code_generic(input.into(), MINUTES_BIT_MASK, MINUTES_POSITION, PARITY_MINUTES_BIT_MASK, MAX_MINUTES.into())
}

/// Extracts the hour out of a dcf77 bitfield
pub fn process_hour(input: u64) -> Result<u8, Error> {
    decode_generic_parity(input, HOUR_MASK, HOUR_POSITION, PARITY_HOUR_BIT_MASK)
}

/// Extracts the minutes out of a dcf77 bitfield
pub fn process_minutes(input: u64) -> Result<u8, Error> {
    decode_generic_parity(input, MINUTES_MASK, MINUTES_POSITION, PARITY_MINUTES_BIT_MASK)
}

#[cfg(test)]
mod tests {
    use crate::to_dcf77;
    use crate::from_dcf77;
    #[test]
    fn test_process_minutes() {
        for fake_input in 0..=0xff {
            match to_dcf77(0, fake_input, 0, 0, 0, 0) {
                Ok(coded_minutes) => {
                    match from_dcf77(coded_minutes) {
                        Ok(decoded_minutes) => {
                            println!("Test Process Minutes : {:X?} - {:X?} - {:X?}", fake_input, coded_minutes, decoded_minutes.minutes);
                            assert!(0 == decoded_minutes.hour);
                            assert!(fake_input == decoded_minutes.minutes);
                            assert!(0 == decoded_minutes.day);
                            assert!(0 == decoded_minutes.month);
                            assert!(0 == decoded_minutes.year);
                            assert!(0 == decoded_minutes.day_of_week);
                        }
                        Err(input) => {
                            //assert!(MAX_HOUR > input)
                            println!("Hello");
                        }
                    }
                }
                Err(coded_hour) => {
                    //assert!(MAX_HOUR < coded_hour)
                    println!("Hello");
                }
            }
        }
    }
    #[test]
    fn test_process_hour() {
        for fake_input in 0..=0xff {
            match to_dcf77(fake_input, 0, 0, 0, 0, 0) {
                Ok(coded_hour) => {
                    match from_dcf77(coded_hour) {
                        Ok(decoded_hour) => {
                            println!("Test Process Hour : {:X?} - {:X?} - {:X?}", fake_input, coded_hour, decoded_hour.hour);
                            assert!(fake_input == decoded_hour.hour);
                            assert!(0 == decoded_hour.minutes);
                            assert!(0 == decoded_hour.day);
                            assert!(0 == decoded_hour.month);
                            assert!(0 == decoded_hour.year);
                            assert!(0 == decoded_hour.day_of_week);
                        }
                        Err(input) => {
                            //assert!(MAX_HOUR > input)
                            println!("Hello");
                        }
                    }
                }
                Err(coded_hour) => {
                    //assert!(MAX_HOUR < coded_hour)
                    println!("Hello");
                }
            }
        }
    }
}
