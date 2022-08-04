use std::io;
use rand::Rng;


fn main() {

    println!("Welcome to Scissor Paper Rock!");

    let name: String = get_user_input("What is your name? ", 10) ;

    println!("\nHello there, {}!", name);

    let mut player_round_wins = 0;
    let mut computer_round_wins = 0;

    let mut player_game_wins = 0;
    let mut computer_game_wins = 0;

    println!("\nThis game is first to 3 wins!\n");
    
    loop {

        if player_round_wins == 3 || computer_round_wins == 3  {
            
            if player_round_wins > computer_round_wins { player_game_wins += 1 } else { computer_game_wins += 1 }
            
            match user_move_selection("Play Again?\n1. Yes\n2. No", 5,1,2){
                    1 => { 
                        println!("\nRematch!\n");
                        computer_round_wins = 0;
                        player_round_wins = 0;
                        continue; 
                    },

                    2 => break,

                    _ => println!("How the heck!")
                }; 
            }
            
        let user_move: u32 = user_move_selection("Choose:\n1.Scissor\n2.Paper\n3.Rock\n", 
            5, 1,3) - 1;

        println!("\nYour move: {}", move_as_string(user_move));

        let computer_move = computer_random_move();
        println!("Computer move: {}\n", move_as_string(computer_move));

        let winner = check_win_loss(computer_move, user_move);

        match winner {
            0 => println!("It's a tie!\n"),
            
            1 => {
                println!("{} won that round!\n", name);
                player_round_wins += 1;
            }

            2 => {
                println!("The Computer won that round!\n");
                computer_round_wins += 1;
            }

            _ => println!("How the heck!")
        }
       
        println!("{} wins: {}\nComputer Wins: {}\n", name, player_round_wins, computer_round_wins);
    }

    println!("\nNumber of games {} won: {}\nNumber of games Computer won: {}", name, player_game_wins, computer_game_wins);
    
    println!("\nThank you for playing!");

}


fn user_move_selection(display_text: &str, input_max_len: usize, min_selection: u32, max_selection: u32) -> u32 {

    loop {
        let selection_str: String = get_user_input(display_text, input_max_len);

        let user_selection: u32 = match selection_str.trim().parse() {
            // case this => do something
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_selection >= min_selection && user_selection <= max_selection 
            { return user_selection } 
        else 
            { println!("Error: Selection out of range.\n"); continue; }
    }    
}


fn get_user_input(display_text: &str, input_max_len: usize) -> String {

    loop {
        let mut input_str: String = String::new();

        println!("{}", display_text);

        io::stdin()
                .read_line(&mut input_str)
                .expect("Failed to read line");

        if input_str.len() <= input_max_len && input_str.len() > 0 {
            return input_str.trim().to_string();

        } else {
            println!("Name exceeds size limit! [{} character(s) total]\n", input_max_len);
        }
    }
}


fn computer_random_move() -> u32 {
    let computer_move: u32 = rand::thread_rng().gen_range(0,3);
    computer_move
}


fn check_win_loss(computer: u32, user: u32) -> u32 {

    let u: usize = user as usize;
    let c: usize = computer as usize;

    // 0 => Tie
    // 1 => player win (computer loss)
    // 2 => computer win (player loss)
    let win_loss_table: [[u32; 3]; 3] = [
        [0, 2, 1],
        [1, 0, 2],
        [2, 1, 0]
    ];

    win_loss_table[c][u]
}


fn move_as_string(selection: u32) -> String {

    let s: usize = selection as usize;

    let move_selection = ["Scissor \\/", "Paper []", "Rock *"];

    move_selection[s].to_string()
}
