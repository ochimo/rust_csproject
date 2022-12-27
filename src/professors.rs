use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};
use std::cmp::Ordering;

use crate::course::Course;

struct Professor {
    prof_id: String,
    prof_name: String,
    courses: RefCell<Vec<Rc<Course>>>,
    left: RefCell<Option<Rc<Professor>>>,
    right: RefCell<Option<Rc<Professor>>>,
}

pub struct Professors {
    root: Option<Rc<Professor>>,
}

impl Professors {
    pub fn build(courses: &Vec<Rc<Course>>) -> Professors {
        let mut professors = Professors {
            root: None
        };

        for course in courses.iter() {
            professors.add_course(course);
        }

        professors
    }

    pub fn add_course(&mut self, course: &Rc<Course>) {
        let mut our_prof = self.find_professor(&course.get_prof_id());
        if our_prof.is_none() {
            our_prof = self.add_professor(course);
        }

        {
            let mut courses: RefMut<_> = our_prof.as_ref().unwrap().courses.borrow_mut();
            courses.push(Rc::clone(course));
        }
    }

    fn find_professor_recursive(item: &Option<Rc<Professor>>, prof_id: &String) -> Option<Rc<Professor>> {
        if item.is_none() {
            return None;
        } else {
            match prof_id.cmp(&item.as_ref().unwrap().prof_id) {
                Ordering::Less => return Self::find_professor_recursive(&*item.as_ref().unwrap().left.borrow(), prof_id),
                Ordering::Equal => return Some(Rc::clone(item.as_ref().unwrap())),
                Ordering::Greater => return Self::find_professor_recursive(&*item.as_ref().unwrap().right.borrow(), prof_id),
            }
        }
    }

    fn find_professor(&mut self, prof_id: &String) -> Option<Rc<Professor>> {
        Self::find_professor_recursive(&self.root, prof_id)
    }

    fn add_professor_recursive(item: &Option<Rc<Professor>>, new_prof: Option<Rc<Professor>>, prof_id: &String) {
        if item.is_none() {
            panic!("should not be none");
        } else {
            match prof_id.cmp(&item.as_ref().unwrap().prof_id) {
                Ordering::Less => {
                    let empty = (*item.as_ref().unwrap().left.borrow()).is_none();
                    if empty {
                        item.as_ref().unwrap().left.replace(new_prof);
                    } else {
                        Self::add_professor_recursive(&*(item.as_ref().unwrap().left.borrow()), new_prof, prof_id);
                    }
                },
                Ordering::Equal => {
                    panic!("should not be duplicates");
                },
                Ordering::Greater => {
                    let empty = (*item.as_ref().unwrap().right.borrow()).is_none();
                    if empty {
                        item.as_ref().unwrap().right.replace(new_prof);
                    } else {
                        Self::add_professor_recursive(&*(item.as_ref().unwrap().right.borrow()), new_prof, prof_id);
                    }
                },
            }
        }
    }

    fn add_professor(&mut self, course: &Rc<Course>) -> Option<Rc<Professor>> {
        // Assumes professor doesn't exist in tree - call find_professor first
        let prof_id = course.get_prof_id();
        let new_prof = Some(Rc::new(Professor {
            prof_id: prof_id.clone(),
            prof_name: course.get_prof_name(),
            courses: RefCell::new(Vec::new()),
            left: RefCell::new(None),
            right: RefCell::new(None),
        }));
        let result = Some(Rc::clone(new_prof.as_ref().unwrap()));
        if self.root.is_none() {
            self.root = new_prof;
        } else {
            Self::add_professor_recursive(&self.root, new_prof, &prof_id);
        }

        result
    }

    pub fn search_professor(&mut self, prof_id: &String) {
        let our_prof = self.find_professor(prof_id);
        if our_prof.is_none() {
            println!("Professor not found");
        } else {
            println!("Name: {}", our_prof.as_ref().unwrap().prof_name);
            let courses: Ref<_> = our_prof.as_ref().unwrap().courses.borrow();
            for course in courses.iter() {
                course.display_course_info();
            }
        }
    }

}
