use rand::Rng;

struct GameResult {
    changed_mind: bool,
    win: bool
}

static VERBOSE: bool = false;

macro_rules! vpr {
    ($($arg:tt)*) => {
        if VERBOSE {
            println!($($arg)*);
        }
    };
}

fn main() {
    vpr!("Play Monty Hall !!!");
    let num_games: i64 = 100000;
    let mode_autoplay: bool = true;

    vpr!("Number of games: {num_games}");
    vpr!("Autoplay: {mode_autoplay}");

    let results = play_x_games(num_games);
    print_stats(results);
}

fn play_x_games(x: i64) -> Vec<GameResult>{
    vpr!("Playing {x} game(s):");

    let mut results: Vec<GameResult> = Vec::new();
    for _ in 0..x {
        let result = play_one_game();
        results.push(result);
    }

    return results;
}

fn play_one_game() -> GameResult {
    let mut rng = rand::thread_rng();

    let door_win = rng.gen_range(0..3);
    let mut door_choice= rng.gen_range(0..3);

    vpr!("Door choice: {:?}", door_choice);

    let door_reveal = (0..3)
    .find(|&door| door != door_win && door != door_choice)
    .unwrap();

    vpr!("Door revealed: {}", door_reveal);
    vpr!("Want to change your mind?");

    let change_mind = rng.gen_bool(0.5);
    if change_mind {

        vpr!("Yes, you changed your mind!");

        door_choice = (0..3)
        .find(|&door| door != door_choice && door != door_reveal)
        .unwrap();

        vpr!("Door choice: {:?}", door_choice);
    }
    let win = door_choice == door_win;
    
    if win { vpr!("You WIN!"); }
    else { vpr!("You LOOSE!"); }

    let result: GameResult = GameResult {changed_mind: change_mind, win: win};

    return result;
}

fn print_stats(results: Vec<GameResult>) {
    let mut changed_mind_num_wins = 0;
    let mut changed_mind_num_games = 0;
    let mut not_changed_mind_num_wins = 0;
    let mut not_changed_mind_num_games = 0;

    for result in &results {
        if result.changed_mind {
            changed_mind_num_games += 1;
            if result.win {
                changed_mind_num_wins += 1;
            }
        }
        else {
            not_changed_mind_num_games += 1;
            if result.win {
                not_changed_mind_num_wins += 1;
            }
        }
    }

    let changed_mind_win_percent = if changed_mind_num_games > 0 {
        (changed_mind_num_wins as f64 / changed_mind_num_games as f64) * 100.0
    }
    else {
        0.0
    };
    let not_changed_mind_win_percentage = if not_changed_mind_num_games > 0 {
        (not_changed_mind_num_wins as f64 / not_changed_mind_num_games as f64) * 100.0
    }
    else {
        0.0
    };

    println!("Percent won when changed mind: {changed_mind_win_percent}");
    println!("Percent won when NOT changed mind: {not_changed_mind_win_percentage}");

}