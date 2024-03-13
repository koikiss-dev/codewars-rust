pub mod opposites_attract_mod{
    pub fn love_func(flower1: u16, flower2: u16) -> bool{
        //let condition: bool = (flower1 % 2 == 0 && flower2 % 2 != 0) || (flower1 % 2 != 0 && flower2 % 2 == 0) ;
        let condition2:bool = flower1 % 2 != flower2 % 2;// mejor opcion
        match condition2 {
            true => true,
            _ => false
        }
        /* if condition {
            true
        }else {
            false
        } */
    }
}