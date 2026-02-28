fn main() {
    let array = vec![2, 9, 12, 18, 21, 26, 32, 37];
    let token = 21;

    let index = binary_search(array, token);

    match index {
        Some(x) => println!("The element is at index {x}"),
        None => println!("The element is not present in the array"),
    }
}

fn binary_search(array: Vec<i32>, token: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        if array[mid] == token {
            return Some(mid);
        } else if token < array[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found_element() {
        let array = vec![2, 9, 12, 18, 21, 26, 32, 37];
        assert_eq!(binary_search(array, 21), Some(4));
    }

    #[test]
    fn test_element_not_found() {
        let array = vec![2, 9, 12, 18, 21, 26, 32, 37];
        assert_eq!(binary_search(array, 15), None);
    }

    #[test]
    fn test_first_element() {
        let array = vec![2, 9, 12, 18, 21, 26, 32, 37];
        assert_eq!(binary_search(array, 2), Some(0));
    }

    #[test]
    fn test_last_element() {
        let array = vec![2, 9, 12, 18, 21, 26, 32, 37];
        assert_eq!(binary_search(array, 37), Some(7));
    }

    #[test]
    fn test_empty_array() {
        let array: Vec<i32> = vec![];
        assert_eq!(binary_search(array, 5), None);
    }

    #[test]
    fn test_single_element_found() {
        let array = vec![5];
        assert_eq!(binary_search(array, 5), Some(0));
    }

    #[test]
    fn test_single_element_not_found() {
        let array = vec![3];
        assert_eq!(binary_search(array, 5), None);
    }
}
