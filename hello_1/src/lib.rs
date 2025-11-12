pub const FREEZING_POINT_F: f32 = 32.0;

pub fn c_to_f(celsius_temp: f32) -> f32 {
    (celsius_temp * (9.0 / 5.0)) + FREEZING_POINT_F
}

pub fn f_to_c(fahrenheit_temp: f32) -> f32 {
    (fahrenheit_temp - FREEZING_POINT_F) * (5.0 / 9.0)
}

pub fn convert(temperature: f32, choice: u8) -> Option<f32> {
    match choice {
        1 => Some(c_to_f(temperature)),
        2 => Some(f_to_c(temperature)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

