use std::fs;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Course {
    year: String,
    department: String,
    course_num: u32,
    course_name: String,
    prof_id: String,
    prof_fname: String,
    prof_lname: String,
}

impl Course {
    pub fn hash(&self, size: usize) -> usize {
        hash_course_num(self.course_num, size)
    }

    pub fn display_course_info(&self) {
        println!("{} {} {} {} {}", self.year, self.course_name, self.course_num, self.prof_fname, self.prof_lname);
    }

    pub fn match_course(&self, course_year: &str, course_num: u32, prof_id: &str) -> bool {
        self.year.eq(course_year) && self.course_num == course_num && self.prof_id.eq(prof_id)
    }

    pub fn get_prof_id(&self) -> String {
        self.prof_id.clone()
    }

    pub fn get_prof_name(&self) -> String {
        self.prof_fname.clone() + " " + &self.prof_lname
    }

}

pub fn hash_course_num(course_num: u32, size: usize) -> usize {
    usize::try_from(course_num).unwrap() % size
}

pub fn read_courses(file_path: &String) -> Vec<Rc<Course>> {
    let mut courses = Vec::new();

    let contents = fs::read_to_string(file_path)
        .expect("Failed to read the course file");

    for line in contents.lines().skip(1) {
        let data: Vec<&str> = line.split(',').collect();
        if data.len() != 7 {
            panic!("Invalid formatting for course file missing columns")
        }
        let course = Rc::new(Course {
            year: data[0].to_string(),
            department: data[1].to_string(),
            course_num: data[2].parse().unwrap(),
            course_name: data[3].to_string(),
            prof_id: data[4].to_string(),
            prof_fname: data[5].to_string(),
            prof_lname: data[6].to_string(),
        });
        courses.push(course);
    }
    dbg!(courses.len());
    courses
}

