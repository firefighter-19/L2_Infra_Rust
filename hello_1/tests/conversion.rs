use hello_1::{c_to_f, convert, f_to_c};

const EPS: f32 = 1e-5;

#[test]
fn c_to_f_converts_zero() {
    let result = c_to_f(0.0);
    assert!((result - 32.0).abs() < EPS);
}

#[test]
fn f_to_c_converts_freezing_point() {
    let result = f_to_c(32.0);
    assert!((result - 0.0).abs() < EPS);
}

#[test]
fn convert_handles_c_to_f() {
    let result = convert(100.0, 1).expect("Expected Some value");
    assert!((result - 212.0).abs() < EPS);
}

#[test]
fn convert_handles_f_to_c() {
    let result = convert(212.0, 2).expect("Expected Some value");
    assert!((result - 100.0).abs() < EPS);
}

#[test]
fn convert_rejects_invalid_choice() {
    assert!(convert(0.0, 99).is_none());
}

