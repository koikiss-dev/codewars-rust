pub mod square_sum_mod{
    pub fn square_sum(vec: Vec<i32>) -> i32 {
        //we can use this = vec.iter().map(|g| g.pow(2)).sum() 
        let pow_digist:Vec<i32> = vec.iter().map(|g| g.pow(2)).collect();

        let mut sum_digits:i32 = 0;

        for k in pow_digist.iter(){
            sum_digits += k;
        }
        
        sum_digits
    }

}