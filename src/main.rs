
use std::process;
use std::env;
use std::io;
use std::rc::Rc;

mod config;
use config::Config;

mod course;
use course::{Course, read_courses};

mod openhash;
use openhash::OpenHash;

mod chaininghash;
use chaininghash::ChainingHash;

mod professors;
use professors::Professors;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    dbg!("file: {}, hash size: {}", &config.file_path, config.hash_size);

    let mut open_hash: Option<OpenHash> = None;
    let mut chaining_hash: Option<ChainingHash> = None;
    let mut professors: Option<Professors> = None;

    let stdin = io::stdin();
    loop {
        println!("
=======Main Menu=======
1. Populate hash tables
2. Search for a course
3. Search for a professor
4. Display all courses
5. Exit
        ");
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        match buffer.as_str().trim_end() {
            "1" => (open_hash, chaining_hash, professors) = populate_hashes(&config),

            "2" => search_course(&open_hash, &chaining_hash),

            "3" => search_professor(&mut professors),

            "4" => display_all_courses(&open_hash, &chaining_hash),

            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Enter 1-5")
        };
    }
}

fn populate_hashes(config: &Config) -> (Option<OpenHash>, Option<ChainingHash>, Option<Professors>) {
    let courses: Vec<Rc<Course>> = read_courses(&config.file_path);
    let open_hash: OpenHash = OpenHash::build(&courses, config.hash_size);
    let chaining_hash: ChainingHash = ChainingHash::build(&courses, config.hash_size);
    let professors: Professors = Professors::build(&courses);

    (Some(open_hash), Some(chaining_hash), Some(professors))
}

fn search_course(open_hash: &Option<OpenHash>, chaining_hash: &Option<ChainingHash>) {
    let stdin = io::stdin();
    let mut course_year = String::new();
    let mut course_num = String::new();
    let mut prof_id = String::new();

    if open_hash.is_none() {
        println!("Populate hash tables first!");
        return;
    }

    println!("Enter the course year (e.g. 2021):");
    stdin.read_line(&mut course_year).unwrap();

    println!("Enter a course number (e.g. 2270):");
    stdin.read_line(&mut course_num).unwrap();

    println!("Enter a Professor's ID (e.g. llytellf):");
    stdin.read_line(&mut prof_id).unwrap();

    let mut found: Option<Rc<Course>> = open_hash.as_ref().unwrap().search(course_year.as_str().trim_end(), course_num.as_str().trim_end().parse().unwrap(), prof_id.as_str().trim_end());
    match found {
        Some(x) => x.display_course_info(),
        None => println!("Course not found via open_hash"),
    };

    found = chaining_hash.as_ref().unwrap().search(course_year.as_str().trim_end(), course_num.as_str().trim_end().parse().unwrap(), prof_id.as_str().trim_end());
    match found {
        Some(x) => x.display_course_info(),
        None => println!("Course not found via chaining_hash"),
    };

}

fn search_professor(professors: &mut Option<Professors>) {
    let stdin = io::stdin();
    let mut prof_id = String::new();

    println!("Enter a Professor's ID (e.g. nscollan0):");
    stdin.read_line(&mut prof_id).unwrap();

    if professors.is_none() {
        println!("Populate hash tables first!");
        return;
    }

    professors.as_mut().unwrap().search_professor(&prof_id.as_str().trim_end().to_string());
}
fn display_all_courses(open_hash: &Option<OpenHash>, chain_hash: &Option<ChainingHash>) {
    let stdin = io::stdin();
    let mut choice = String::new();

    if open_hash.is_none() || chain_hash.is_none() {
        println!("Populate hash tables first!");
        return;
    }

    loop {
        println!("Which hash table would you like to display the courses for (O=Open Addressing, C=Chaining)?");
        stdin.read_line(&mut choice).unwrap();
        match choice.as_str().trim_end() {
            "O" => { open_hash.as_ref().unwrap().display_all_courses(); break; },
            "C" => { chain_hash.as_ref().unwrap().display_all_courses(); break; },
            _ => println!("Select O or C")
        };
    }
}