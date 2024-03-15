pub mod terminal_game_mod{
    //roll = a tirada
    pub fn move_hero(position: u32, roll:u32) -> u32{
        let move_hero: u32 = roll*2;

        /* if position != 0 {
            position + move_hero
        }else{
            move_hero
        } */

        match position {
            p if p != 0 => position + move_hero,
            _ => move_hero
        }
        //bueno viendo los demas ejemplos solo debia poner position + roll * 2 xd 
    }
}