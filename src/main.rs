use std::io;
use rand::Rng;
use std::collections::HashMap;

enum Game {
    Bool(bool),
    String(String)
}

fn main() {
    println!("Play Monty Hall !!!");
    let mut input_num_games = String::new();
    let mut input_mode_autoplay = String::new();
    println!("How many games do you want to play?");
    io::stdin().read_line(&mut input_num_games).expect("Please enter a value larger than 0");
    println!("Do you want the games to be auto-played by the machine?");
    io::stdin().read_line(&mut input_mode_autoplay).expect("Please enter true or false");
    
    let num_games: i64 = input_num_games.trim().parse().unwrap();
    let mode_autoplay: bool = input_mode_autoplay.trim().parse().unwrap();

    println!("Number of games: {num_games}");
    println!("Autoplay: {mode_autoplay}");

    play_x_games(num_games);
}

fn play_x_games(x: i64) {

    let mut games_played: Vec<HashMap<String, Game>> = vec![];
    

    for _i in 0..x {
        play_one_game(&games_played);
    }
}


fn play_one_game(games_played: &Vec<HashMap<String, Game>>) {

    let mut rng = rand::thread_rng();
    let doors_possible = vec! [
        vec![false, false, true],
        vec![false , true, false],
        vec![true ,false , false]
    ];
    let mut doors = &doors_possible[rng.gen_range(0..doors_possible.len())];
    let mut door_choice = rng.gen_range(0..doors.len());

    if !(doors[door_choice]) {
        let change_mind = rng.gen_bool(0.5);
        if change_mind {
            doors.remove(door_choice);
            door_choice = rng.gen_range(0..doors.len());
            if doors[door_choice] {

            }
        }
    }

    let mut new_game_entry: HashMap<String, Game> = HashMap::new();
    new_game_entry.insert("name".to_string(), Game::String("Test".to_string()));
    games_played.push(new_game_entry);
    //println!("Win: {door_win}, Choice: {door_choice}");
}