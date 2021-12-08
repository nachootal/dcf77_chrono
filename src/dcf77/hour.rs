use std::io::Error;
use super::*;

// As documented: https://www.cyber-sciences.com/wp-content/uploads/2019/01/TN-103_DCF77.pdf
/// Mask for the hour [0..24) in the DCF77 bit field
const HOUR_BIT_MASK: u64 = 0x3F;

/// Position of the parity bit for the hour in the DCF77 bit field
const PARITY_HOUR_BIT_MASK: u64 = 1 << 24;

/// Position of the bits for the hour in the DCF77 bit field
const HOUR_POSITION: u8 = 25;

/// Mask for the hour aligned in the DCF77 bit field
const HOUR_MASK: u64 = HOUR_BIT_MASK << HOUR_POSITION;

/// Maximum value for the hour in a DCF77 bit field
const MAX_HOUR: u32 = 24;

/// Codes a given hour [0..24) into DCF77 bit field
pub fn code_hour(input: u32) -> Result<u64, Error> {
    let section = SectionInBitfield {data_bit_mask: HOUR_BIT_MASK,
                                     data_position: HOUR_POSITION,
                                     parity_mask: PARITY_HOUR_BIT_MASK,
                                     max_data: MAX_HOUR};
    code_dcf77(input, section)
}

/// Extracts the hour out of a dcf77 bitfield
pub fn process_hour(input: u64) -> Result<u32, Error> {
    println!("Process Hour: 0x{:X} - 0x{:X}- {:}- 0x{:X}", input, HOUR_MASK, HOUR_POSITION, PARITY_HOUR_BIT_MASK);
    let section = SectionInBitfield {data_bit_mask: HOUR_BIT_MASK,
                                     data_position: HOUR_POSITION,
                                     parity_mask: PARITY_HOUR_BIT_MASK,
                                     max_data: MAX_HOUR};
    decode_dcf77(input, section)
}

/// Mask for the minutes [0..59) in the DCF77 bit field
const MINUTES_BIT_MASK: u64 = 0x7F;

/// Position of the parity bit for the minutes in the DCF77 bit field
const PARITY_MINUTES_BIT_MASK: u64 = 1 << 31;

/// Position of the bits for the minutes in the DCF77 bit field
const MINUTES_POSITION: u8 = 32;

/// Mask for the minutes aligned in the DCF77 bit field
const MINUTES_MASK: u64 = MINUTES_BIT_MASK << MINUTES_POSITION;

/// Maximum value for the minutes in a DCF77 bit field
const MAX_MINUTES: u32 = 60;

/// Codes a given minutes [0..59) into DCF77 bit field
pub fn code_minutes(input: u32) -> Result<u64, Error> {
    let section = SectionInBitfield {data_bit_mask: MINUTES_BIT_MASK,
                                     data_position: MINUTES_POSITION,
                                     parity_mask: PARITY_MINUTES_BIT_MASK,
                                     max_data: MAX_MINUTES};
    code_dcf77(input, section)
}

/// Extracts the minutes out of a dcf77 bitfield
pub fn process_minutes(input: u64) -> Result<u32, Error> {
    println!("Process minutes: 0x{:X} - 0x{:X}- {:}- 0x{:X}", input, MINUTES_MASK, MINUTES_POSITION, PARITY_MINUTES_BIT_MASK);
    let section = SectionInBitfield {data_bit_mask: MINUTES_BIT_MASK,
                                     data_position: MINUTES_POSITION,
                                     parity_mask: PARITY_MINUTES_BIT_MASK,
                                     max_data: MAX_MINUTES};
    decode_dcf77(input, section)
}

#[cfg(test)]
mod tests {
    use crate::to_dcf77;
    use crate::from_dcf77;
    use chrono::prelude::*;
    #[test]
    fn test_process_minutes() {
        let original_test_time = Utc::now() - chrono::Duration::minutes(Utc::now().time().minute().into());
        for fake_input in 0..=0xff {
            let test_time = original_test_time + chrono::Duration::minutes(fake_input.into());
            match to_dcf77(test_time) {
                Ok(coded_minutes) => {
                    match from_dcf77(coded_minutes) {
                        Ok(decoded_minutes) => {
                            println!("Minutes: Invented Hour: {:?} - Decoded Hour: {:?}", test_time.time(), decoded_minutes.time());
                            assert!(test_time.time() == decoded_minutes.time())
                        }
                        Err(input) => {
                            println!("Error on decoding {:} - 0x{:X} to dcf77 {:?}", fake_input, coded_minutes, input);
                        }
                    }
                }
                Err(coded_hour) => {
                    println!("Error on creating the dcf77 {:?}", coded_hour);
                }
            }
        }
    }
    #[test]
    fn test_process_hour() {
        let original_test_time = Utc::now() - chrono::Duration::hours(Utc::now().time().hour().into());
        for fake_input in 0..=0xff {
            let test_time = original_test_time + chrono::Duration::hours(fake_input.into());
            match to_dcf77(test_time) {
                Ok(coded_hour) => {
                    match from_dcf77(coded_hour) {
                        Ok(decoded_hour) => {
                            println!("Hour: Invented Hour: {:?} - Decoded Hour: {:?}", test_time.time(), decoded_hour.time());
                            assert!(test_time.time() == decoded_hour.time())
                        }
                        Err(input) => {
                            println!("Error on decoding {:} - 0x{:X} to dcf77 {:?}", fake_input, coded_hour, input);
                        }
                    }
                }
                Err(coded_hour) => {
                    println!("Error on creating the dcf77 {:?}", coded_hour);
                }
            }
        }
    }
}
