use unitconv::domain::units::Unit;

#[test]
fn test_temperature_conversion() {
    let result = Unit::Celcius.convert(&Unit::Kelvin, 0.0).unwrap();
    assert_eq!(result, "0 °C = 273.15 K");
}

#[test]
fn test_length_conversion() {
    let result = Unit::Cm.convert(&Unit::Km, 16000.0).unwrap();
    assert_eq!(result, "16000 cm = 0.16 km");
}

#[test]
fn test_category_mismatch_error() {
    let err = Unit::Cm.convert(&Unit::Celcius, 1.0).unwrap_err();
    assert!(err.contains("Tidak dapat mengonversi satuan yang berbeda kategori"));
}
