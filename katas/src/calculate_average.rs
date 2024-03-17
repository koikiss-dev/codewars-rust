pub mod find_average_mod{
    pub fn find_average(slice: &[f64]) -> f64 {
        let average = slice.iter().sum::<f64>() as f64 / slice.len() as f64;
       
        if !slice.is_empty(){
            average
        }else{
            0 as f64
        }
        
    }
}