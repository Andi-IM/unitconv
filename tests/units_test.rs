use unitconv::domain::units::Unit;

#[test]
fn test_celcius_fahrenheit_1() {
    let result = Unit::Celcius.convert(&Unit::Fahrenheit, 87.0).unwrap();
    assert_eq!(result, "87 °C = 188.6 °F");
}

#[test]
fn test_celcius_fahrenheit_2() {
    let result = Unit::Celcius.convert(&Unit::Fahrenheit, 60.0).unwrap();
    assert_eq!(result, "60 °C = 140 °F");
}

#[test]
fn test_celcius_kelvin_1() {
    let result = Unit::Celcius.convert(&Unit::Kelvin, 23.0).unwrap();
    assert_eq!(result, "23 °C = 296.15 K");
}

#[test]
fn test_celcius_kelvin_2() {
    let result = Unit::Celcius.convert(&Unit::Kelvin, 76.0).unwrap();
    assert_eq!(result, "76 °C = 349.15 K");
}

#[test]
fn test_fahrenheit_celcius_1() {
    let result = Unit::Fahrenheit.convert(&Unit::Celcius, 77.0).unwrap();
    assert_eq!(result, "77 °F = 25 °C");
}

#[test]
fn test_fahrenheit_celcius_2() {
    let result = Unit::Fahrenheit.convert(&Unit::Celcius, 24.0).unwrap();
    assert_eq!(result, "24 °F = -4.44 °C");
}

#[test]
fn test_kelvin_celcius_1() {
    let result = Unit::Kelvin.convert(&Unit::Celcius, 66.0).unwrap();
    assert_eq!(result, "66 K = -207.15 °C");
}

#[test]
fn test_kelvin_celcius_2() {
    let result = Unit::Kelvin.convert(&Unit::Celcius, 128.0).unwrap();
    assert_eq!(result, "128 K = -145.15 °C");
}

#[test]
fn test_cm_inch() {
    let result = Unit::Cm.convert(&Unit::Inch, 30.0).unwrap();
    assert_eq!(result, "30 cm = 11.81 inch");
}

#[test]
fn test_cm_km() {
    let result = Unit::Cm.convert(&Unit::Km, 160000.0).unwrap();
    assert_eq!(result, "160000 cm = 1.6 km");
}

#[test]
fn test_cm_miles() {
    let result = Unit::Cm.convert(&Unit::Miles, 15200.0).unwrap();
    assert_eq!(result, "15200 cm = 0.09 miles");
}

#[test]
fn test_inch_cm() {
    let result = Unit::Inch.convert(&Unit::Cm, 11.0).unwrap();
    assert_eq!(result, "11 inch = 27.94 cm");
}

#[test]
fn test_inch_km() {
    let result = Unit::Inch.convert(&Unit::Km, 1520000.0).unwrap();
    assert_eq!(result, "1520000 inch = 38.61 km");
}

#[test]
fn test_inch_miles() {
    let result = Unit::Inch.convert(&Unit::Miles, 1520000.0).unwrap();
    assert_eq!(result, "1520000 inch = 23.99 miles");
}

#[test]
fn test_km_cm() {
    let result = Unit::Km.convert(&Unit::Cm, 0.5).unwrap();
    assert_eq!(result, "0.5 km = 50000 cm");
}

#[test]
fn test_km_inch() {
    let result = Unit::Km.convert(&Unit::Inch, 2.4).unwrap();
    assert_eq!(result, "2.4 km = 94488.19 inch");
}

#[test]
fn test_km_miles() {
    let result = Unit::Km.convert(&Unit::Miles, 13.0).unwrap();
    assert_eq!(result, "13 km = 8.08 miles");
}

#[test]
fn test_miles_cm() {
    let result = Unit::Miles.convert(&Unit::Cm, 10.0).unwrap();
    assert_eq!(result, "10 miles = 1609344 cm");
}

#[test]
fn test_miles_inch() {
    let result = Unit::Miles.convert(&Unit::Inch, 2.0).unwrap();
    assert_eq!(result, "2 miles = 126720 inch");
}

#[test]
fn test_miles_km() {
    let result = Unit::Miles.convert(&Unit::Km, 6.0).unwrap();
    assert_eq!(result, "6 miles = 9.66 km");
}

#[test]
fn test_category_mismatch_error() {
    let err = Unit::Cm.convert(&Unit::Celcius, 1.0).unwrap_err();
    assert!(err.contains("Tidak dapat mengonversi satuan yang berbeda kategori"));
}
