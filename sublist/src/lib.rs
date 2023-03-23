#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {

    match (a, b){
        _ if a == b => Comparison::Equal,
        (a, b) if a.len() < b.len() && contain(a,b) => Comparison::Sublist,
        (a, b) if a.len() > b.len() && contain(b,a) => Comparison::Superlist,
        _ => Comparison::Unequal
    }
}

pub fn contain<T: PartialEq>(a: &[T], b: &[T]) -> bool{
        a.is_empty() || b.windows(a.len()).any(|s| s == a)
}