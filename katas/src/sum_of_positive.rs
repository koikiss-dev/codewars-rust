pub mod sum_of_positive_mod{
    pub fn positive_sum(slice: &[i32]) -> i32{
        /* let mut sum: i32 = 0;

        for &arr in slice.iter(){
            
            if arr > 0{
                
                sum+=arr;
            }
        }
        
        sum */
        //forma mas corta sin el for si no con el iter
        slice.iter().filter(|x|  x.is_positive()).sum()
    }
}