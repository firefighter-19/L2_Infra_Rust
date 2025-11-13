#![warn(clippy::all, clippy::pedantic)]

use std::cmp::Ordering;

fn main() {
    match binary_search(&[1, 2, 3, 4, 5], 3) {
        Some(index) => println!("Target found at index: {index}"),
        None => println!("Target not found"),
    }
}

fn binary_search(arr: &[i32], target: i32) -> Option<usize> {

    if arr.is_empty() { return None; }

    let mut left = 0;
    let mut right = arr.len() - 1;
    
    while left <= right {

        let mid = usize::midpoint(left, right);

        match arr[mid].cmp(&target) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid.checked_sub(1)?,
            Ordering::Equal => return Some(mid),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_binary_search_found() {
        let arr = [1, 2, 3, 4, 5];
        let target = 3;
        let result = binary_search(&arr, target);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [1, 2, 3, 4, 5];
        let target = 6;
        let result = binary_search(&arr, target);
        assert!(result.is_none());
    }

    #[test]
    fn test_binary_search_empty_array() {
        let arr = [];
        let target = 1;
        let result = binary_search(&arr, target);
        assert!(result.is_none());
    }

    #[test]
    fn test_binary_search_single_element() {
        let arr = [1];
        let target = 1;
        let result = binary_search(&arr, target);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn binary_search_negative_numbers() {
        let arr = [-5, -3, -1, 1, 3, 5];
        let target = -100;
        let result = binary_search(&arr, target);
        assert!(result.is_none());
    }
}