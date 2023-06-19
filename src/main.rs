//-------------MODULE IMPORT AREA -------------------//
mod ui_comps;
mod game_logic;
mod sub_funcs;

//-------------IMPORT FUNCTIONS AREA-----------------//
use std::io::stdin;
use ui_comps::{NON_FAMOUS_ERROR_MSG,WRONG_USER_INPUT_ERROR_MSG,main_dashboard_text};
use sub_funcs::sub_function_router;

//--------------MAIN RUNDOWN LOOP--------------------//
fn main(){
    loop{
    main_dashboard_text();
    let mut user_passer_to_functions=String::new();
    stdin().read_line(&mut user_passer_to_functions).expect(NON_FAMOUS_ERROR_MSG);
    let user_passer_to_functions=user_passer_to_functions.trim();
    match user_passer_to_functions.parse::<i32>() {
        Ok(number) => {sub_function_router(number)}
        Err(_) => {println!("{}",WRONG_USER_INPUT_ERROR_MSG)}
    };
}


}
