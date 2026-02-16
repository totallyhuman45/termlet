use colored::*;
use std::{fs,process};
use rfd::FileDialog;
use std::io::{self, stdout, Write};


use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    cursor::{Hide, Show},
};

use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
struct Term {
    term: String,
    definition: String,
}



#[derive(Debug)]
struct Settings {
    percent_acuracy: i32,
    capitlization_specific: bool,
    whitespace_trim: bool,
    acent_marks_removal: bool,
    first_letter_hint: bool,
    streak_tracking: bool,
    provide_error_marking: bool,
    desplay_score_on_response: bool,
    include_first_row: bool,
}
impl Term{
    fn question(& self, settings:&Settings) -> bool {
        println!("{}",self.term);
        io::stdout().flush().unwrap();

        // user prompt
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        // get corectness


        true
    }
}




impl Settings{
    fn defalt() -> Self {
        Self {
            percent_acuracy: 90,
            capitlization_specific: false,
            whitespace_trim: true,
            acent_marks_removal: false,
            first_letter_hint: false,
            streak_tracking: true,
            provide_error_marking: true,
            desplay_score_on_response: true,
            include_first_row: true,
        }
    }

    fn ask_setting_bool(&mut self, field: &str, question: &str){
        // ask question
        print!("true or false, {}",question);
        io::stdout().flush().unwrap();


        // prompt user
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let answer: bool = input.trim().eq_ignore_ascii_case("true");

        match field{
            "capitlization_specific" => self.capitlization_specific = answer,
            "whitespace_trim" => self.whitespace_trim = answer,
            "acent_marks_removal" => self.acent_marks_removal = answer,
            "first_letter_hint" => self.first_letter_hint = answer,
            "streak_tracking" => self.streak_tracking = answer,
            "provide_error_marking" => self.provide_error_marking = answer,
            "desplay_score_on_response" => self.desplay_score_on_response = answer,
            "include_first_row" => self.include_first_row = answer,
            _ => panic!("invalid field prompting"),
        }
    }

    fn ask_setting_percentage(&mut self){
        print!("percent acuracy to be correct: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let answer: i32 = input.trim().replace("%", "").parse().expect("failed to read percentage");

        self.percent_acuracy = answer;
    }
}



fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    //enter alternate screen
    execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All))?;



    println!("{}", "quiz".red().bold());
    print!("{}", "please enter your csv export: ".bright_blue());
    // make sure the print staments output is not buffered untill after file reading
    io::stdout().flush().unwrap();


    let file:Option<std::path::PathBuf> = FileDialog::new().add_filter("CSV Files", &["csv"]).pick_file();
    let path:std::path::PathBuf;

    match file {
        Some(x) =>{ 
            println!("Selected file: {}", x.display());
            path = x;
        },
        None => {
            println!("No file selected");
            execute!(stdout, Show, LeaveAlternateScreen)?;
            process::exit(1);
        },
    }

    let mut terms:Vec<Term> = match load_csv(path){
        Ok(result) => result,
        Err(error_massage) => {
            execute!(stdout, Show, LeaveAlternateScreen)?;
            println!("{}",error_massage);
            process::exit(1);
        }
    };

    //data at this point is loaded we just need to get settings
    let mut settings = Settings::defalt();
    // keep defalt or enter custom
    print!("true or false, enter your oun settings: ");
    io::stdout().flush().unwrap();


    // prompt user
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let answer: bool = input.trim().eq_ignore_ascii_case("true");

    if answer{
        settings.ask_setting_percentage();
        settings.ask_setting_bool("capitlization_specific","Check if answers must match capitalization: ");
        settings.ask_setting_bool("whitespace_trim","Ignore extra spaces in answers: ");
        settings.ask_setting_bool("acent_marks_removal","Treat letters without accents as correct: ");
        settings.ask_setting_bool("first_letter_hint","Show the first letter as a hint: ");
        settings.ask_setting_bool("streak_tracking","Keep track of correct answer streaks: ");
        settings.ask_setting_bool("provide_error_marking","Mark wrong answers visibly: ");
        settings.ask_setting_bool("desplay_score_on_response","Show score after each answer: ");
        settings.ask_setting_bool("include_first_row","include the first row in questions: ");
    }

    // start question sequence
    levenshtein_distance("cut"," cut");

    loop {
        terms[0].question(&settings);
    }

    sleep(Duration::from_secs(10));




    // Restore original screen
    execute!(stdout, Show, LeaveAlternateScreen)?;

    

   

    Ok(())
}

fn load_csv(path:std::path::PathBuf) -> Result<Vec<Term>,String>{
    let file_contents:String;
    match fs::read_to_string(&path) {
        Ok(contents) => {
            file_contents = contents;
            println!("File contents:\n{}", file_contents);
        }
        Err(e) => {
            return Err(format!("Error reading file: {}", e))
        }
    }
    let file_lines: Vec<&str> = file_contents.split("\n").collect::<Vec<&str>>();
    let mut term_list: Vec<Term> = vec![];

    let mut tmp_seperated: Vec<&str>;
    for (i,x) in file_lines.iter().enumerate(){
        if x.trim() == ""{
            continue;
        }
        tmp_seperated = x.split(",").collect::<Vec<&str>>();
        if tmp_seperated.len() != 2{
            return Err(format!("File line {} contains to many or to few definitions", i + 1 as usize))
        }
        let term = match tmp_seperated.get(0){
            Some(x) => x,
            None => return Err(format!("File line {} contains to many or to few definitions", i + 1 as usize))
        };
        let definition = match tmp_seperated.get(1){
            Some(x) => x,
            None => return Err(format!("File line {} contains to many or to few definitions", i + 1 as usize))
        };
        term_list.push(Term{term: term.to_string(),definition: definition.to_string()});
    }
    Ok(term_list)
}

fn levenshtein_distance(correct:&str,awnser:&str){
    let mut grid:Vec<Vec<i32>> = vec![];
    let mut awnser_vec: Vec<char> = awnser.chars().collect::<Vec<char>>();
    let mut correct_vec: Vec<char> = correct.chars().collect::<Vec<char>>();

    awnser_vec.insert(0, ' ');
    correct_vec.insert(0, ' ');

    // fill out the vecs with there colums and filler 0s 
    let mut colum:Vec<i32>;
    for (i,x) in awnser_vec.iter().enumerate(){
        colum = vec![];
        colum.push(i as i32);
        for n in 1..correct_vec.len(){
            colum.push(0);
        }
        grid.push(colum);
    }
    //fix first colum
    for (i,x) in grid[0].clone().iter().enumerate(){
        grid[0][i] = i as i32;
    }

    let mut cost:i32;
    let mut left:i32;
    let mut top: i32;
    let mut diagonal:i32;
    for (i,x) in grid.clone().iter().enumerate(){
        for (j,t) in x.iter().enumerate(){
            if i as i32 == 0 || j as i32 == 0{
                continue;
            }
            if awnser_vec[i] == correct_vec[j]{
                cost = 0;
            }else{
                cost = 1;
            }
            left = grid[i -1][j];
            top = grid[i][j - 1];
            diagonal = grid[i -1][j - 1];

            left = left + 1;
            top = top + 1;
            diagonal = diagonal + cost;
            grid[i][j] = *[left,top,diagonal].iter().min().unwrap();
            
        }
    }




    let 
    loop{

    }


    println!("{:?}",grid);

}