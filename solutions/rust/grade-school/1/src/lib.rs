use std::collections::{BTreeMap, BinaryHeap};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    grades_students: BTreeMap<u32, BinaryHeap<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades_students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.grades_students.contains_key(&grade) {
            let students = self.grades_students.get_mut(&grade).unwrap();
            students.push(student.to_owned());
        } else {
            let mut students = BinaryHeap::new();
            students.push(student.to_owned());
            self.grades_students.insert(grade, students);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut ans: Vec<u32> = self.grades_students.keys().map(|v| *v).collect();
        ans.sort();
        ans
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let value = self.grades_students.get(&grade);
        match value {
            Some(students) => {
                let mut ans = students.iter().map(|s| s.clone()).collect::<Vec<String>>();
                ans.sort();
                ans
            }
            None => {
                vec![]
            }
        }
    }
}