fn merge(left_vector: Vec<i32>, right_vector: Vec<i32>) -> Vec<i32> {
    let (left_len, right_len) = (left_vector.len(), right_vector.len());
    let (mut left, mut right) = (0, 0);
    let mut merged_vector: Vec<i32> = Vec::new();
    while left < left_len && right < right_len {
        if left_vector[left] < right_vector[right] {
            merged_vector.push(left_vector[left]);
            left += 1;
        } else {
            merged_vector.push(right_vector[right]);
            right += 1;
        }
    }
    while left < left_len {
        merged_vector.push(left_vector[left]);
        left += 1;
    }
    while right < right_len {
        merged_vector.push(right_vector[right]);
        right += 1;
    }
    return merged_vector;
}

fn merge_sort(unsorted_vector: Vec<i32>) -> Vec<i32> {
    let len = unsorted_vector.len();

    match len {
        1 => {
            unsorted_vector.to_vec()
        }
        2 => {
            let mut sorted_vector = vec![unsorted_vector[0], unsorted_vector[1]];
            if sorted_vector[0] > sorted_vector[1] {
                (sorted_vector[1], sorted_vector[0]) = (sorted_vector[0], sorted_vector[1]);
            }
            sorted_vector
        }
        _ => {
            let mid = len / 2;
            let left = merge_sort(unsorted_vector[..mid].to_vec());
            let right = merge_sort(unsorted_vector[mid..].to_vec());
            merge(left, right)
        }
    }
}
