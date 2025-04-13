//! Encoding is in the form: ASCD_DATE_NAME
//!
//! Where ASCD ensures that the file is sorted by descending chronological date
//! by inverting the date and storing it as letters.

use chrono::Datelike;
use chrono::NaiveDate;

pub fn encode_key(date: NaiveDate, title: &str) -> String {
    // convert the date into something lexicographically ascending
    let year = 9999 - date.year();
    let month = 99 - (date.month0() + 1);
    let day = 99 - (date.day0() + 1);
    let numbers = format!("{year}-{month}-{day}");

    const NUMBER_TO_DIGIT_DIFFERENCE: u32 = 17;
    let ascending_date = numbers
        .chars()
        .map(|c| {
            if c.is_numeric() {
                char::from_u32(c as u32 + NUMBER_TO_DIGIT_DIFFERENCE).unwrap()
            } else {
                c
            }
        })
        .collect::<String>();

    format!("{}_{}_{}", ascending_date, date.format("%Y-%m-%d"), title)
}

pub fn decode_key(key: String) -> (NaiveDate, String) {
    let parts = key.splitn(3, '_').collect::<Vec<_>>();
    let [_ascd, date, name] = parts.try_into().expect("key should be in 3 parts");

    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("date fmt should be correct");
    (date, name.to_owned())
}

#[test]
fn key_roundtrip() {
    let original_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let original_title = "Test";

    let encoded = encode_key(original_date, original_title);
    let (decoded_date, decoded_title) = decode_key(encoded);

    assert_eq!(original_date, decoded_date);
    assert_eq!(original_title, decoded_title);
}
