use std::collections::HashSet;

pub fn is_unique(s: String) -> bool {
    let len = s.len();
    if len > 128 {
        return false;
    }

    let mut chars = HashSet::new();

    for c in s.chars() {
        if chars.contains(&c) {
            return false;
        } else {
            chars.insert(c);
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_is_unique() {
        assert_eq!(is_unique(String::from("Daniel")), true);
    }

    #[test]
    fn it_is_not_unique() {
        assert_eq!(is_unique(String::from("Daniela")), false);
    }

    #[test]
    fn it_is_unique_when_empty() {
        assert_eq!(is_unique(String::from("")), true);
    }

    #[test]
    fn it_is_not_unique_when_has_more_than_128_chars() {
        assert_eq!(is_unique(String::from("gxSJ2nXDda5WiRq6ow4HNGpxfvS4AzMi2im7UjmfZ3kwGtNf69d69uGns6dL12qeP664hP5sjml85JlxwtA05vRhkK2XnXK3EPPsqHuJmvvkrZByHvYXVV83n7YxcN2h9")), false);
    }
}
