//MAIN IMPORT AREA
use crate::game_logic::{
    game_scoring_system, match_win_system, random_computer_char_gen, user_char_gen, NO_OF_MATCHES,
    NO_OF_ROUNDS,
};
use crate::ui_comps::{
    main_credits_text, main_help_text, status_game_print, NON_FAMOUS_ERROR_MSG,
    WRONG_USER_INPUT_ERROR_MSG,
};
use std::io::stdin;
use std::process::exit;

//MAIN GAME
pub fn game() {
    let mut iter_match: u32 = 1;
    let mut match_mark_user: u32 = 0;
    let mut match_mark_computer: u32 = 0;
    while iter_match <= NO_OF_MATCHES {
        let mut iter_round: u32 = 1;
        let mut user_round_mark: u32 = 0;
        let mut computer_round_mark: u32 = 0;
        while iter_round <= NO_OF_ROUNDS {
            status_game_print(iter_match, iter_round);
            let mut user_response = String::new();
            stdin()
                .read_line(&mut user_response)
                .expect(NON_FAMOUS_ERROR_MSG);
            let user_response = user_response.trim();
            let user_number = user_response.parse::<u32>().unwrap_or(0);
            let score_bool: bool =
                game_scoring_system(user_char_gen(user_number), random_computer_char_gen());
            if score_bool {
                user_round_mark += 1
            } else {
                computer_round_mark += 1
            }
            iter_round += 1;
        }
        println!(
            "\n\nEND OF MATCH {} YOU👨-{}  COMPUTER📳-{} ",
            iter_match, user_round_mark, computer_round_mark
        );
        if match_win_system(user_round_mark, computer_round_mark) {
            match_mark_user += 1;
        } else {
            match_mark_computer += 1;
        }
        iter_match += 1;
        println!(
            "\n\nMATCH WON COUNT YOU👨-{}  COMPUTER📳-{} ",
            match_mark_user, match_mark_computer
        );
    }
    if match_mark_user > match_mark_computer {
        println!("\n\nYAY YOU HAVE WON THE GAME 🏆🏆🏆")
    } else {
        println!("\n\nYOU LOSER THE COMPUTER WON 😥😥😥")
    }
}

//NON ESSENTIALS//
pub fn help() {
    main_help_text();
}
pub fn credits() {
    main_credits_text()
}
pub fn quit() {
    exit(0);
}
//FUNCTION ROUTER//
pub fn sub_function_router(option_var: i32) {
    if option_var == 1 {
        game()
    } else if option_var == 2 {
        help()
    } else if option_var == 3 {
        credits()
    } else if option_var == 4 {
        quit()
    } else {
        println!("{}", WRONG_USER_INPUT_ERROR_MSG)
    }
}
