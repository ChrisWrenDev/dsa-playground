fn linear_search<T: PartialEq>(arr: &[T], target: T) -> Option<usize> {
    for (index, item) in arr.iter().enumerate() {
        if *item == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    let target = 30;

    match linear_search(&numbers, target) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found in the list"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        let data = vec![5, 10, 15, 20];
        assert_eq!(linear_search(&data, 15), Some(2));
    }

    #[test]
    fn test_not_found() {
        let data = vec![1, 2, 3, 4];
        assert_eq!(linear_search(&data, 5), None);
    }

    #[test]
    fn test_empty_array() {
        let data: Vec<i32> = vec![];
        assert_eq!(linear_search(&data, 1), None);
    }

    #[test]
    fn test_first_element() {
        let data = vec![7, 8, 9];
        assert_eq!(linear_search(&data, 7), Some(0));
    }

    #[test]
    fn test_last_element() {
        let data = vec![1, 2, 3];
        assert_eq!(linear_search(&data, 3), Some(2));
    }
}
