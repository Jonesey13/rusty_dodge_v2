/*
Defines Basic Enemy Behaviour
*/


use super::player::Player;


pub trait Enemy{
    #[allow(unused_variables)]
    fn update_position(&mut self, game_time: f64, player: &Player){}
}
