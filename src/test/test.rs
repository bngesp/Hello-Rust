//#[derive(Debug)]
//enum DIRECTION{
//    N,E,S,W
//}
//
//enum PlayerAction {
//    Move{
//        direction:DIRECTION,
//        speed:u8
//    },
//    Wait,
//    Attack(DIRECTION)
//}
//
//fn main() {
//    let mut test = 23i32;
//    match fais_quelque_chose(test) {
//        Some(s) => println!("{}", &s),
//        None => {} // rien à afficher donc on ne fait rien
//    }
//    test = 13;
//    if let Some(s) = fais_quelque_chose(test) {
//        println!("{}", &s)
//    }
//
//    // deuxieme partie avec les enumerations
//    let simulation_player_action = PlayerAction::Attack(DIRECTION::W);
//
//    match simulation_player_action {
//        PlayerAction::Wait => println!("le joueur doit s'arreter"),
//        PlayerAction::Move {direction, speed} => { println!("ce joueur marche avec les infomations suivante direction = {:?} et vitesse {}", direction, speed)},
//        PlayerAction::Attack(dir) => { println!("ce joueur attack vers la direction {:?}", dir)}
//    };
//
//}
//
//fn fais_quelque_chose(i: i32) -> Option<String> {
//    if i < 100 {
//        println!("la valuer est {:}", i);
//        return Some("variable inférieure à 10".to_owned());
//    } else {
//        println!("la valuer est {:}", i);
//        return None;
//    }
//}

struct Player {
    name: String,
    iq: u8,
    friends:u8
}

impl Player {
    fn with_name(name: &str) -> Player{
        Player{
            name: name.to_string(),
            iq: 100,
            friends: 100
        }
    }

    // fn with_name_and_iq(name: &Str, val: u8) -> Player{
    //     Player{
    //         name: name.to_string(),
    //         iq: u8::from(val),
    //         friends: 100
    //     }
    // }
    fn get_friends(&self) -> u8{
        self.friends
    }
    fn set_friends(&mut self, count: u8){
        self.friends  = count;
    }
}


fn test(){
    let mut player = Player::with_name("Dave");
    // let mut player2 = Player::with_name_and_iq("Dave", 23);
    player.set_friends(23);
    println!("{}'s friends count: {}", player.name, player.get_friends());
    // another way to call instance methods.
    let _ = Player::get_friends(&player);
}
