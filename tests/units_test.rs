use approx::assert_relative_eq;
use tempfile::NamedTempFile;
use unitconv::domain::{
    records::{ConversionRecord, load_history, save_to_history},
    units::Unit,
};
use serial_test::serial;

fn init_test_env() {
    // Create a temprorary file that will act as the history storage.
    // We convert it into a TempPath se we can take ownership of the path independent of the
    // file handle, then call `keep` to prevent the file from being deleted when the TempPath
    // goes out of scope. This guarantees the file lives for the entire duration of the test
    // suite and avoids it being removed prematurely, which previously caused empty histories.
    let temp_file = NamedTempFile::new().expect("Failed to create temp history file");
    let temp_path = temp_file.into_temp_path();
    let persisted_path = temp_path
        .keep()
        .expect("Failed to persist temp history file");
    unsafe { std::env::set_var("UNITCONV_HISTORY_PATH", &persisted_path) };

    // print variable has been set up
    println!(
        "UNITCONV_HISTORY_PATH: {}",
        std::env::var("UNITCONV_HISTORY_PATH").unwrap()
    );
}

#[test]
fn test_celcius_fahrenheit_1() {
    init_test_env();
    let result = Unit::Celsius.convert(&Unit::Fahrenheit, 87.0).unwrap();
    assert_eq!(result, "87 °C = 189 °F");
}

#[test]
fn test_celcius_fahrenheit_2() {
    init_test_env();
    let result = Unit::Celsius.convert(&Unit::Fahrenheit, 60.0).unwrap();
    assert_eq!(result, "60 °C = 140 °F");
}

#[test]
fn test_celcius_kelvin_1() {
    init_test_env();
    let result = Unit::Celsius.convert(&Unit::Kelvin, 23.0).unwrap();
    assert_eq!(result, "23 °C = 296 K");
}

#[test]
fn test_celcius_kelvin_2() {
    init_test_env();
    let result = Unit::Celsius.convert(&Unit::Kelvin, 76.0).unwrap();
    assert_eq!(result, "76 °C = 349 K");
}

#[test]
fn test_fahrenheit_celcius_1() {
    init_test_env();
    let result = Unit::Fahrenheit.convert(&Unit::Celsius, 77.0).unwrap();
    assert_eq!(result, "77 °F = 25 °C");
}

#[test]
fn test_fahrenheit_celcius_2() {
    init_test_env();
    let result = Unit::Fahrenheit.convert(&Unit::Celsius, 24.0).unwrap();
    assert_eq!(result, "24 °F = -4 °C");
}

#[test]
fn test_kelvin_celcius_1() {
    init_test_env();
    let result = Unit::Kelvin.convert(&Unit::Celsius, 66.0).unwrap();
    assert_eq!(result, "66 K = -207 °C");
}

#[test]
fn test_kelvin_celcius_2() {
    init_test_env();
    let result = Unit::Kelvin.convert(&Unit::Celsius, 128.0).unwrap();
    assert_eq!(result, "128 K = -145 °C");
}

#[test]
fn test_cm_inch() {
    init_test_env();
    let result = Unit::Cm.convert(&Unit::Inch, 30.0).unwrap();
    assert_eq!(result, "30 cm = 11.811 inch");
}

#[test]
fn test_cm_km() {
    init_test_env();
    let result = Unit::Cm.convert(&Unit::Km, 160000.0).unwrap();
    assert_eq!(result, "160000 cm = 1.6 km");
}

#[test]
fn test_cm_miles() {
    init_test_env();
    let result = Unit::Cm.convert(&Unit::Miles, 15200.0).unwrap();
    assert_eq!(result, "15200 cm = 0.0944 miles");
}

#[test]
fn test_inch_cm() {
    init_test_env();
    let result = Unit::Inch.convert(&Unit::Cm, 11.0).unwrap();
    assert_eq!(result, "11 inch = 27.94 cm");
}

#[test]
fn test_inch_km() {
    init_test_env();
    let result = Unit::Inch.convert(&Unit::Km, 1520000.0).unwrap();
    assert_eq!(result, "1520000 inch = 38.608 km");
}

#[test]
fn test_inch_miles() {
    init_test_env();
    let result = Unit::Inch.convert(&Unit::Miles, 1520000.0).unwrap();
    assert_eq!(result, "1520000 inch = 23.9899 miles");
}

#[test]
fn test_km_cm() {
    init_test_env();
    let result = Unit::Km.convert(&Unit::Cm, 0.5).unwrap();
    assert_eq!(result, "0.5 km = 50000 cm");
}

#[test]
fn test_km_inch() {
    init_test_env();
    let result = Unit::Km.convert(&Unit::Inch, 2.4).unwrap();
    assert_eq!(result, "2.4 km = 94488.189 inch");
}

#[test]
fn test_km_miles() {
    init_test_env();
    let result = Unit::Km.convert(&Unit::Miles, 13.0).unwrap();
    assert_eq!(result, "13 km = 8.0778 miles");
}

#[test]
fn test_miles_cm() {
    init_test_env();
    let result = Unit::Miles.convert(&Unit::Cm, 10.0).unwrap();
    assert_eq!(result, "10 miles = 1609344 cm");
}

#[test]
fn test_miles_inch() {
    init_test_env();
    let result = Unit::Miles.convert(&Unit::Inch, 2.0).unwrap();
    assert_eq!(result, "2 miles = 126720 inch");
}

#[test]
fn test_miles_km() {
    init_test_env();
    let result = Unit::Miles.convert(&Unit::Km, 6.0).unwrap();
    assert_eq!(result, "6 miles = 9.6561 km");
}

#[test]
fn test_category_mismatch_error() {
    init_test_env();
    let err = Unit::Cm.convert(&Unit::Celsius, 1.0).unwrap_err();
    assert!(err.contains("Tidak dapat mengonversi satuan yang berbeda kategori"));
}

#[test]
#[serial]
fn test_is_data_saved() {
    init_test_env();
    let record = ConversionRecord {
        from: "cm".to_owned(),
        to: "inch".to_owned(),
        value: 10.0,
        result: 27.94,
        display_text: "10 cm = 27.94 inch".to_string(),
    };
    let _ = save_to_history(record.clone());
    let history = load_history();
    let history_vec = history.unwrap();
    // println!("{:?}", history_vec);
    assert_eq!(history_vec.len(), 1);
    assert_relative_eq!(history_vec[0].result, record.result);
}
