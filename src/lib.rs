#![warn(missing_docs)]
//! The dcf_chrono library offers translation between chrono and DCF77 standard
//!
//! The library offers two main methods:
//! - to_dcf77
//! - from_dcf77

use std::io::Error;
use chrono::prelude::*;
mod dcf77;

/// Struct that represents the DCF77 information
///
/// This struct contains a chrono DateTime<Utc> field and the metadata
/// that the DCF77 offers.
///
///
///
/// # Examples
/// ```
///use chrono::prelude::*;
///use dcf77_chrono::DCF77;
///let output = DCF77 {
///    date: Utc.ymd(2021,
///                11,
///                12).and_hms(11,
///                            22,
///                            0),
///    antenna: true,
///    announce_daily_saving_time: false,
///    daily_saving_time: false,
///    standard_time: true,
///    bit_leap_second: false};
///    assert!(output.date.year() == 2021);
///    assert!(output.date.month() == 11);
///    assert!(output.date.day() == 12);
///    assert!(output.antenna == true);
///    assert!(output.announce_daily_saving_time == false);
///    assert!(output.daily_saving_time == false);
///    assert!(output.standard_time == true);
///    assert!(output.bit_leap_second == false);
/// ```
#[derive(Copy, Clone)]
pub struct DCF77 {
    /// Date in UTC timezone
    pub date: DateTime<Utc>,
    /// Antenna bit (always 0)
    pub antenna: bool,
    /// Announce Bit for Daylight Saving Time (DST) Switching
    pub announce_daily_saving_time: bool,
    /// If this bit is set, Daylight Saving Time (DST) is Active
    pub daily_saving_time: bool,
    /// If this bit is set, Standard Time is Active
    pub standard_time: bool,
    /// Announce Bit for Leap Second
    pub bit_leap_second: bool
}

/// Decodes the date and metadata out of a dcf77 bit field
///
/// A DCF77 bitfield is given as input and a DCF77 struct is returned when successful
///
/// # Examples
/// ```
///use chrono::prelude::*;
///use dcf77_chrono::*;
///let original_test_time = Utc::now() - chrono::Duration::minutes(Utc::now().time().minute().into());
///let test_time = DCF77 {
///    date: original_test_time,
///    antenna: false,
///    announce_daily_saving_time: false,
///    daily_saving_time: false,
///    standard_time: false,
///    bit_leap_second: false
///};
///match to_dcf77(test_time) {
///    Ok(coded_minutes) => {
///        match from_dcf77(coded_minutes) {
///            Ok(decoded_minutes) => {
///                assert!(test_time.date.time() == decoded_minutes.date.time())
///            }
///            Err(input) => {
///                println!("Error on decoding 0x{:X} to dcf77 {:?}", coded_minutes, input);
///            }
///        }
///    }
///    Err(coded_hour) => {
///        println!("Error on creating the dcf77 {:?}", coded_hour);
///    }
///}
/// ```
pub fn from_dcf77(input: u64) -> Result<DCF77, Error> {
    let processed_hour = dcf77::hour::process_hour(input)?;
    let processed_minutes = dcf77::hour::process_minutes(input)?;
    let processed_day = dcf77::date::process_day(input)?;
    let processed_month = dcf77::date::process_month(input)?;
    let processed_year = dcf77::date::process_year(input)? as i32;
    let output = DCF77 {
        date: Utc.with_ymd_and_hms(processed_year,
                                    processed_month,
                                    processed_day,
                                    processed_hour,
                                    processed_minutes,
                                    0).single().unwrap(),
        antenna: dcf77::metadata::process_antenna(input),
        announce_daily_saving_time: dcf77::metadata::process_announce_daylight_saving_switch(input),
        daily_saving_time: dcf77::metadata::process_daylight_saving(input),
        standard_time: dcf77::metadata::process_standard_time(input),
        bit_leap_second: dcf77::metadata::process_bit_leap_second(input)};
    Ok(output)
}

/// Encodes a dcf77 bit field containing the information of a DCF77 struct
///
/// A DCF77 struct is given as input and a DCF77 bitfield is returned when successful
///
/// # Examples
/// ```
///use chrono::prelude::*;
///use dcf77_chrono::*;
///let original_test_time = Utc::now() - chrono::Duration::minutes(Utc::now().time().minute().into());
///let test_time = DCF77 {
///    date: original_test_time,
///    antenna: false,
///    announce_daily_saving_time: false,
///    daily_saving_time: false,
///    standard_time: false,
///    bit_leap_second: false
///};
///match to_dcf77(test_time) {
///    Ok(coded_minutes) => {
///        match from_dcf77(coded_minutes) {
///            Ok(decoded_minutes) => {
///                assert!(test_time.date.time() == decoded_minutes.date.time())
///            }
///            Err(input) => {
///                println!("Error on decoding 0x{:X} to dcf77 {:?}", coded_minutes, input);
///            }
///        }
///    }
///    Err(coded_hour) => {
///        println!("Error on creating the dcf77 {:?}", coded_hour);
///    }
///}
/// ```
pub fn to_dcf77(dcf_data: DCF77) -> Result<u64, Error> {
    let given_date = dcf_data.date.date_naive();
    let given_time = dcf_data.date.time();
    let coded_hour = dcf77::hour::code_hour(given_time.hour())?;
    let coded_minutes = dcf77::hour::code_minutes(given_time.minute())?;
    let coded_day = dcf77::date::code_day(given_date.day())?;
    let coded_day_of_week = dcf77::date::code_day_of_the_week(given_date.weekday().number_from_monday().try_into().unwrap())?;
    let coded_month = dcf77::date::code_month(given_date.month())?;
    let coded_year = dcf77::date::code_year(given_date.year())?;
    let coded_antenna = dcf77::metadata::code_antenna(dcf_data.antenna);
    let coded_announce_daily_saving_time = dcf77::metadata::code_announce_daylight_saving_switching(dcf_data.announce_daily_saving_time);
    let coded_daily_saving_time = dcf77::metadata::code_daylight_saving(dcf_data.announce_daily_saving_time);
    let coded_standard_time = dcf77::metadata::code_standard_time(dcf_data.standard_time);
    let coded_bit_leap_second = dcf77::metadata::code_bit_leap_second(dcf_data.bit_leap_second);
    Ok(coded_hour |
        coded_minutes |
        coded_day |
        coded_day_of_week |
        coded_month |
        coded_year |
        coded_antenna |
        coded_announce_daily_saving_time |
        coded_daily_saving_time |
        coded_standard_time |
        coded_bit_leap_second)
}
