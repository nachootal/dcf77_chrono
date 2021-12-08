use std::io::Error;
use chrono::prelude::*;
mod dcf77;

/// Decodes the hour and minutes out of a dcf77 bit field
pub fn from_dcf77(input: u64) -> Result<DateTime<Utc>, Error> {
    let processed_hour = dcf77::hour::process_hour(input)?;
    let processed_minutes = dcf77::hour::process_minutes(input)?;
    let processed_day = dcf77::date::process_day(input)?;
    //let processed_day_of_week = dcf77::date::process_day_of_week(input);
    let processed_month = dcf77::date::process_month(input)?;
    let processed_year = dcf77::date::process_year(input)? as i32;
    println!("FROM Date: {:}.{:}.{:} - Hour: {:}:{:}", processed_year, processed_month, processed_day, processed_hour, processed_minutes);
    Ok(Utc.ymd(processed_year, processed_month, processed_day).and_hms(processed_hour, processed_minutes, 0))
}

/// Produces a bit field with the given hour and minutes
pub fn to_dcf77(date: DateTime<Utc>) -> Result<u64, Error> {
    let given_date = date.date().naive_local();
    let given_time = date.time();
    println!("Date: {:?} : Hour: {:?}", given_date, given_time);
    let coded_hour = dcf77::hour::code_hour(given_time.hour())?;
    let coded_minutes = dcf77::hour::code_minutes(given_time.minute())?;
    let coded_day = dcf77::date::code_day(given_date.day())?;
    //let coded_day_of_week = dcf77::date::code_day_of_the_week(day_of_week)?;
    let coded_month = dcf77::date::code_month(given_date.month())?;
    let coded_year = dcf77::date::code_year(given_date.year())?;
    println!("TO Date: 0x{:X}.0x{:X}.0x{:X} - Hour: 0x{:X}:0x{:X}", coded_year, coded_month, coded_day, coded_hour, coded_minutes);
    Ok(coded_hour | coded_minutes | coded_day | coded_month | coded_year)
}
