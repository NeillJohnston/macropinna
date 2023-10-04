pub fn try_f64_to_i32(x: f64) -> Option<i32> {
    if !x.is_finite() { return None; }

    let x = x.round();
    if x > i32::MAX as f64 { return None; }
    if x < i32::MIN as f64 { return None; }

    return Some(x as i32);
}