
use std::rc::Rc;

use crate::course::{Course, hash_course_num};

pub struct OpenHash {
    open_hash: Vec<Option<Rc<Course>>>,
    hash_size: usize,
}

impl OpenHash {
    pub fn build(courses: &Vec<Rc<Course>>, hash_size: usize) -> OpenHash {
        let mut open_hash: Vec<Option<Rc<Course>>> = Vec::new();
        open_hash.resize(usize::try_from(hash_size).unwrap(), None);

        let mut collisions = 0;
        let mut search_ops = 0;

        for course in courses.iter() {
            let mut hash = course.hash(hash_size);
            let mut i = 0;
            let mut had_collission = false;
            loop {
                if open_hash[hash] == None {
                    open_hash[hash] = Some(course.clone());
                    break;
                }
                search_ops += 1;
                had_collission = true;

                i += 1;
                hash = (hash + i*i) % hash_size;
            }
            if had_collission {
                collisions += 1;
            }
        }

        println!("[OPEN ADDRESSING] Hash table populated");
        println!("--------------------------------------------------------");
        println!("Collisions using open addressing: {}", collisions);
        println!("Search operations using open addressing: {}", search_ops);

        OpenHash {
            open_hash: open_hash,
            hash_size: hash_size,
        }
    }

    pub fn search(&self, course_year: &str, course_num: u32, prof_id: &str) -> Option<Rc<Course>> {
        let mut result: Option<Rc<Course>> = None;

        let mut i = 0;
        let mut search_ops = 0;
        let mut hash = hash_course_num(course_num, self.hash_size);

        println!("[OPEN ADDRESSING] Search for a course");
        println!("-------------------------------------");
        loop {
            search_ops += 1;
            if self.open_hash[hash] == None {
                break;
            }
            else if self.open_hash[hash].as_ref()?.match_course(course_year, course_num, prof_id) {
                result = Some(self.open_hash[hash].as_ref()?.clone());
                println!("Search operations using open addressing: {}", search_ops);
                break;
            }
            i += 1;
            hash = (hash + i*i) % self.hash_size;

            // @@@ TODO: Is this correct to exit not found?
            if i > self.hash_size {
                dbg!("Cannot find element");
                break;
            }
        }

        result
    }

    pub fn display_all_courses(&self) {
        println!("[OPEN ADDRESSING] display_all_courses()");
        println!("--------------------------------");

        self.open_hash.iter().for_each(|course| {
            match course {
                Some(x) => x.display_course_info(),
                None => {},
            };
        });
    }
}