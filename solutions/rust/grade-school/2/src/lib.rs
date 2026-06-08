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
        self.grades_students
            .entry(grade)
            .or_insert(BinaryHeap::new())
            .push(student.to_owned());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades_students.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let value = self.grades_students.get(&grade);
        match value {
            Some(students) => students.clone().into_sorted_vec(),
            None => {
                vec![]
            }
        }
    }
}