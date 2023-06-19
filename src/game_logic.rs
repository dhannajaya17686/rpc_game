use rand::Rng;

use crate::ui_comps::{WRONG_USER_INPUT_ERROR_MSG,WRONG_USER_INPUT_MSG_INSULT};


pub const CHAR_TUPLE:[char;3]=['🪨','🧻','✃'];
pub const NO_OF_MATCHES:u32=3;
pub const NO_OF_ROUNDS:u32=10;


pub fn random_computer_char_gen()->char{
    let index:usize=rand::thread_rng().gen_range(0..CHAR_TUPLE.len());
    CHAR_TUPLE[index]
}

pub fn game_scoring_system(user_gen_char:char,computer_gen_char:char)->bool{
    let mut bool_status:bool=false;
    if user_gen_char==computer_gen_char{
        bool_status=false;
    }
    else if user_gen_char==CHAR_TUPLE[0]{
       if computer_gen_char==CHAR_TUPLE[1]{
           bool_status=false;
       }
       else if computer_gen_char==CHAR_TUPLE[2]{
           bool_status=true
       }
    }
    else if user_gen_char==CHAR_TUPLE[1]{
       if computer_gen_char==CHAR_TUPLE[0]{
           bool_status=true;
       }
       else if computer_gen_char==CHAR_TUPLE[2]{
           bool_status=false;
       }

       }
    else if user_gen_char==CHAR_TUPLE[2]{
       if computer_gen_char==CHAR_TUPLE[0]{
           bool_status=false;
       }
       else if computer_gen_char==CHAR_TUPLE[1]{
           bool_status=true;
       }
    }
    else{
        bool_status=false;
    }
    println!("YOUR MOVE: {}  COMPUTER MOVE: {}  YOUR MOVE IS: {}",user_gen_char,computer_gen_char,bool_status);
    bool_status
}

pub fn user_char_gen(user_input_num:u32)->char{
    let mut user_char:char=CHAR_TUPLE[0];
    if user_input_num==1{
        user_char=CHAR_TUPLE[0];
    }
    else if user_input_num==2 {
        user_char=CHAR_TUPLE[1];
    }
    else if user_input_num==3{
        user_char=CHAR_TUPLE[2];
    }
    else{
        user_char='❌';
        println!("{}",WRONG_USER_INPUT_ERROR_MSG);
        println!("{}",WRONG_USER_INPUT_MSG_INSULT);
    }
    user_char
}

pub fn match_win_system(user_marks:u32,computer_marks:u32)->bool{
    if user_marks >=computer_marks{return true}
    else{return false}
}
