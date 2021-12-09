use std::io::Error;
use chrono::prelude::*;
mod dcf77;

/// Struct that represents the DCF77 information
#[derive(Copy, Clone)]
pub struct DCF77 {
    pub date: DateTime<Utc>,
    pub antenna: bool,
    pub announce_daily_saving_time: bool,
    pub daily_saving_time: bool,
    pub standard_time: bool,
    pub bit_leap_second: bool
}

/// Decodes the hour and minutes out of a dcf77 bit field
pub fn from_dcf77(input: u64) -> Result<DCF77, Error> {
    let processed_hour = dcf77::hour::process_hour(input)?;
    let processed_minutes = dcf77::hour::process_minutes(input)?;
    let processed_day = dcf77::date::process_day(input)?;
    //TODO: let processed_day_of_week = dcf77::date::process_day_of_week(input);
    let processed_month = dcf77::date::process_month(input)?;
    let processed_year = dcf77::date::process_year(input)? as i32;
    let output = DCF77 {
        date: Utc.ymd(processed_year,
                    processed_month,
                    processed_day).and_hms(processed_hour,
                                            processed_minutes,
                                            0),
        antenna: dcf77::metadata::process_antenna(input),
        announce_daily_saving_time: dcf77::metadata::process_announce_daylight_saving_switch(input),
        daily_saving_time: dcf77::metadata::process_daylight_saving(input),
        standard_time: dcf77::metadata::process_standard_time(input),
        bit_leap_second: dcf77::metadata::process_bit_leap_second(input)};
    Ok(output)
}

/// Produces a bit field with the given hour and minutes
pub fn to_dcf77(dcf_data: DCF77) -> Result<u64, Error> {
    let given_date = dcf_data.date.date().naive_local();
    let given_time = dcf_data.date.time();
    let coded_hour = dcf77::hour::code_hour(given_time.hour())?;
    let coded_minutes = dcf77::hour::code_minutes(given_time.minute())?;
    let coded_day = dcf77::date::code_day(given_date.day())?;
    //TODO: let coded_day_of_week = dcf77::date::code_day_of_the_week(day_of_week)?;
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
        coded_month |
        coded_year |
        coded_antenna |
        coded_announce_daily_saving_time |
        coded_daily_saving_time |
        coded_standard_time |
        coded_bit_leap_second)
}
