/// What should the type of _function be?
pub fn map<T, F, E>(input: Vec<T>, mut apply: F) -> Vec<E>
where
    F: FnMut(T) -> E,
{
    let mut output = Vec::with_capacity(input.len());

    for value in input {
        output.push(apply(value));
    }

    output
}
