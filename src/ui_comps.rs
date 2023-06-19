
pub fn main_dashboard_text(){
    println!("------------  ROCK PAPER SCSSIORS WITH COMPUTER ------------");
    println!("|    🪨                                        ✂️          |");
    println!("|           🧻                           🧻                |");
    println!("|                    ✂️           🪨                       |");
    println!("-----------------------------------------------------------");
    println!("| ⌛-START THE GAME -TYPE 1                                |");
    println!("| 🙋-HELP ME        -TYPE 2                                |");
    println!("| ℹ️ -CREDITS        -TYPE 3                                |");
    println!("| ❌-EXIT           -TYPE-4                                |");
    println!("------------------------------------------------------------");
}

pub fn main_help_text(){
    println!("-----HELP --------------------------------------------------");
    println!("\nTHIS GAME IS VERY SIMPLE! ");
    println!("🔥-YOU HAVE TO PLAY 3 MATHES WITH THE COMPUTER ");
    println!("🔥-EACH MATCH CONTAINS 10 ROUNDS");
    println!("🔥-WHOEVER WINS 2 3 RD OF THE MATCHES WINS!!");
    println!("🔥-I DON'T NEED TO EXPLAIN THE RULES OF ROCK PAPER SCISSORS RIGHT?");
    println!("🔥-SO TRY YOUR LUCK!")
}

pub fn main_credits_text(){
    println!("-----CREDITS-------------------------------------------------");
    println!("⚒️-VERSION 0.0.1");
    println!("⚒️-MADE BY SPARK TECH ");
}

pub fn status_game_print(match_id:u32,round_id:u32){
    println!("MATCH--{} ROUND-{} \t YOU👨 VS COMPUTER📳",match_id,round_id);
}

pub const NON_FAMOUS_ERROR_MSG:&str="SORRY! AN UNEXPECTD ERROR OCCURRED ";
pub const WRONG_USER_INPUT_ERROR_MSG:&str="SORRY! YOU HAVE ENTERED AN WRONG INPUT";
pub const WRONG_USER_INPUT_MSG_INSULT:&str="WARNING--❗❗❗ THE COMPUTER GETS +1 MARK FOR YOUR WRONG INPUT ";