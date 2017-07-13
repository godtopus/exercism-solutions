#[derive(Debug, PartialEq)]
pub enum Comparison {
    Sublist,
    Superlist,
    Equal,
    Unequal
}

pub fn sublist<T: PartialEq + PartialOrd + Clone + Copy>(a: &[T], b: &[T]) -> Comparison {
    return if a.len() <= b.len() {
        compare(&a.to_vec(), &b.to_vec())
    } else {
        let comparison = compare(&b.to_vec(), &a.to_vec());
        match comparison {
            Comparison::Sublist => Comparison::Superlist,
            _ => comparison
        }
    }
}

fn compare<T: PartialEq + PartialOrd + Clone + Copy>(a: &Vec<T>, b: &Vec<T>) -> Comparison {
    if a.is_empty() && b.len() > 0 {
        return Comparison::Sublist
    } else if a.is_empty() && b.is_empty() {
        return Comparison::Equal
    }

    let match_found = b.windows(a.len())
                       .map(|window| window.into_iter().zip(a).skip_while(|&(left, right)| left == right).count() == 0)
                       .any(|m| m);

    if match_found {
        if a.len() == b.len() {
            Comparison::Equal
        } else {
            Comparison::Sublist
        }
    } else {
        Comparison::Unequal
    }
}