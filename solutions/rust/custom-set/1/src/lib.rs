#[derive(Debug)]
pub struct CustomSet<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    pub data: Vec<T>,
}

impl<T> PartialEq for CustomSet<T>
where
    T: PartialEq + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        for d in self.data.iter() {
            if !other.data.contains(d) {
                return false;
            }
        }

        for o in other.data.iter() {
            if !self.data.contains(o) {
                return false;
            }
        }

        true
    }
}

impl<T> Eq for CustomSet<T> where T: PartialEq + Eq {}

impl<T> CustomSet<T>
where
    T: PartialEq + Eq + Clone,
{
    pub fn new(input: &[T]) -> Self {
        let mut data = vec![];

        for d in input {
            if !data.contains(d) {
                data.push(d.clone());
            }
        }
        Self { data }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for d in self.data.iter() {
            if !other.data.contains(d) {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        for o in other.data.iter() {
            if self.data.contains(o) {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut both_data = vec![];
        for o in other.data.iter() {
            if self.data.contains(o) {
                both_data.push(o.clone());
            }
        }

        Self { data: both_data }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut diff_data = vec![];
        for d in self.data.iter() {
            if !other.data.contains(d) {
                diff_data.push(d.clone());
            }
        }
        Self { data: diff_data }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut all_data = self.data.clone();
        for o in other.data.iter() {
            if !all_data.contains(o) {
                all_data.push(o.clone());
            }
        }

        Self { data: all_data }
    }
}
