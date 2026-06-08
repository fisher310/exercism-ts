#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let la = _first_list.len();
    let lb = _second_list.len();
    if la < lb {
        for i in 0..lb - la + 1 {
            if list_eq(_first_list, &_second_list[i..(i+la)]) {
                return Comparison::Sublist;
            }
        }
    } else if la == lb {
        if list_eq(_first_list, _second_list) {
            return Comparison::Equal;
        }
    } else {
        for i in 0..(la - lb) + 1 {
            if list_eq(&_first_list[i..(i + lb)], _second_list) {
                return Comparison::Superlist;
            }
        }
    }

    Comparison::Unequal
}

fn list_eq<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}
