pub fn calculate_heating_time(original_watt: f64, original_time: f64, new_watt: f64) -> (f64, f64) {
    let new_time = original_time * (original_watt / new_watt);

    let new_minutes = (new_time / 60.0).floor();
    let new_seconds = new_time % 60.0;

    (new_minutes, new_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_heating_time() {
        let original_watt = 1000.0;
        let original_time = 120.0;
        let new_watt = 600.0;

        let (new_minutes, new_seconds) = calculate_heating_time(original_watt, original_time, new_watt);

        assert_eq!(new_minutes, 200.0);
        assert_eq!(new_seconds, 0.0);
    }
}