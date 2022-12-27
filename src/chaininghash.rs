use std::rc::Rc;

use crate::course::{Course, hash_course_num};

struct Chain {
    element: Rc<Course>,
    next: Option<Box<Chain>>,
}

pub struct ChainingHash {
    chain_hash: Vec<Option<Chain>>,
    hash_size: usize,
}

impl ChainingHash {
    pub fn build(courses: &Vec<Rc<Course>>, hash_size: usize) -> ChainingHash {
        let mut chain_hash: Vec<Option<Chain>> = Vec::new();
        for _ in 0..hash_size {
            chain_hash.push(None);
        }

        let mut collisions = 0;
        let mut search_ops = 0;

        for course in courses.iter() {
            let new_chain = Chain {
                element: course.clone(),
                next: None
            };

            let hash = course.hash(hash_size);
            if chain_hash[hash].is_none() {
                chain_hash[hash] = Some(new_chain);
                continue;
            }
            collisions += 1;

            // @@@ TODO: Check for duplicates - but not required in this dataset
            let mut walk_chain: &mut Chain = &mut chain_hash[hash].as_mut().unwrap();
            loop {
                search_ops += 1;
                if walk_chain.next.is_some() {
                    walk_chain = walk_chain.next.as_mut().unwrap();
                } else {
                    walk_chain.next = Some(Box::new(new_chain));
                    break;
                }
            }
        }

        println!("[CHAINING] Hash table populated");
        println!("--------------------------------------------------------");
        println!("Collisions using chaining: {}", collisions);
        println!("Search operations using chaining: {}", search_ops);

        ChainingHash {
            chain_hash: chain_hash,
            hash_size: hash_size,
        }
    }

    pub fn search(&self, course_year: &str, course_num: u32, prof_id: &str) -> Option<Rc<Course>> {
        let mut result: Option<Rc<Course>> = None;
        let mut search_ops = 0;
        let hash = hash_course_num(course_num, self.hash_size);

        println!("[CHAINING] Search for a course");
        println!("-------------------------------------");

        if self.chain_hash[hash].is_none() {
            dbg!("Cannot find element in hash array");
        } else {
            let mut walk_chain: &Chain = &self.chain_hash[hash].as_ref().unwrap();
            loop {
                search_ops += 1;
                if walk_chain.element.match_course(course_year, course_num, prof_id) {
                    result = Some(walk_chain.element.clone());
                    println!("Search operations using chaining: {}", search_ops);
                    break;
                }

                if walk_chain.next.is_some() {
                    walk_chain = walk_chain.next.as_ref().unwrap();
                } else {
                    dbg!("Cannot find element in chain");
                    break;
                }
            }
        }

        result
    }

    pub fn display_all_courses(&self) {
        println!("[CHAINING] display_all_courses()");
        println!("--------------------------------");

        self.chain_hash.iter().for_each(|chain| {
            match chain {
                Some(x) => {
                    x.element.display_course_info();
                    let mut n = x.next.as_ref();
                    while n.is_some() {
                        n.unwrap().element.display_course_info();
                        n = n.unwrap().next.as_ref();
                    }
                },
                None => {},
            }
        });
    }
}