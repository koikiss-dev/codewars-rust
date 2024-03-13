pub mod beginner_series_mod{
   pub fn cockroach_speed(s:f64) -> i64{
    let to_cm = s * (100000 as f64 /3600 as f64);
    to_cm as i64
   }
}