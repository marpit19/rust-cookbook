pub fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;

    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);

    merge(arr, mid);
}

pub fn merge(arr: &mut [i32], mid: usize) {
    // TODO: Recursively sort left and right halves
    // TODO: Implement merge logic
    // Hint: You'll need temporary storage for one half
    // and merge back into the original array
    let left = arr[0..mid].to_vec();

    let mut i = 0;
    let mut j = mid;
    let mut k = 0;

    while i < left.len() && j < arr.len() {
        if left[i] <= arr[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_array() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![3, 3, 1, 1, 2, 2];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_large_array() {
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        let expected: Vec<i32> = (0..1000).collect();
        merge_sort(&mut arr);
        assert_eq!(arr, expected);
    }
}
