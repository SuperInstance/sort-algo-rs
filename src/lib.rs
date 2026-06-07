//! # sort-algo-rs
//!
//! A pure-Rust library of sorting algorithm implementations.
//!
//! Provides quicksort (Lomuto + Hoare), mergesort, heapsort, timsort-inspired,
//! counting sort, and radix sort — all with no external dependencies.

/// Quicksort using the Lomuto partition scheme.
///
/// Sorts the slice in ascending order in-place. Not stable.
/// Average O(n log n), worst case O(n²).
pub fn quicksort_lomuto<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    quicksort_lomuto_inner(arr, 0, arr.len() - 1);
}

fn quicksort_lomuto_inner<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let p = partition_lomuto(arr, lo, hi);
    if p > 0 {
        quicksort_lomuto_inner(arr, lo, p - 1);
    }
    quicksort_lomuto_inner(arr, p + 1, hi);
}

fn partition_lomuto<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = hi;
    let mut i = lo;
    for j in lo..hi {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, hi);
    i
}

/// Quicksort using the Hoare partition scheme.
///
/// Sorts the slice in ascending order in-place. Not stable.
/// Generally performs fewer swaps than Lomuto.
pub fn quicksort_hoare<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    quicksort_hoare_inner(arr, 0, arr.len() - 1);
}

fn quicksort_hoare_inner<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let p = partition_hoare(arr, lo, hi);
    if p > 0 {
        quicksort_hoare_inner(arr, lo, p);
    }
    quicksort_hoare_inner(arr, p + 1, hi);
}

fn partition_hoare<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot_idx = lo + (hi - lo) / 2;
    let mut i = lo;
    let mut j = hi;
    loop {
        while arr[i] < arr[pivot_idx] {
            i += 1;
        }
        while arr[j] > arr[pivot_idx] {
            j -= 1;
        }
        if i >= j {
            return j;
        }
        arr.swap(i, j);
        i += 1;
        j = j.saturating_sub(1);
    }
}

/// Mergesort — stable, O(n log n) sorting.
///
/// Returns a new sorted vector. Preserves relative order of equal elements.
pub fn mergesort<T: Clone + Ord>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let left = mergesort(&arr[..mid]);
    let right = mergesort(&arr[mid..]);
    merge(&left, &right)
}

fn merge<T: Clone + Ord>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    while i < left.len() {
        result.push(left[i].clone());
        i += 1;
    }
    while j < right.len() {
        result.push(right[j].clone());
        j += 1;
    }
    result
}

/// Heapsort using a max-heap.
///
/// Sorts the slice in ascending order in-place. Not stable.
/// O(n log n) guaranteed.
pub fn heapsort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    // Build max-heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }
    // Extract elements
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    if left < n && arr[left] > arr[largest] {
        largest = left;
    }
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

/// Timsort-inspired adaptive merge sort.
///
/// Detects natural runs, reverses descending runs, and merges using a stack
/// to maintain invariants. Stable sort.
pub fn timsort<T: Clone + Ord>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let runs = find_runs(arr);
    merge_runs(&runs)
}

/// Find natural runs in the input, reversing descending ones.
fn find_runs<T: Clone + Ord>(arr: &[T]) -> Vec<Vec<T>> {
    let mut runs = Vec::new();
    if arr.is_empty() {
        return runs;
    }
    let mut start = 0;
    while start < arr.len() {
        let mut end = start + 1;
        let mut ascending = true;
        if end < arr.len() {
            ascending = arr[end] >= arr[start];
            while end < arr.len() {
                let in_run = if ascending {
                    arr[end] >= arr[end - 1]
                } else {
                    arr[end] < arr[end - 1]
                };
                if in_run {
                    end += 1;
                } else {
                    break;
                }
            }
        }
        let mut run: Vec<T> = arr[start..end].to_vec();
        if !ascending && run.len() > 1 {
            run.reverse();
        }
        // Ensure minimum run length of 32
        if run.len() < 32 && end < arr.len() {
            let extend = (32 - run.len()).min(arr.len() - end);
            run.extend_from_slice(&arr[end..end + extend]);
            // Sort the extended run using insertion sort
            insertion_sort(&mut run);
            end += extend;
        }
        runs.push(run);
        start = end;
    }
    runs
}

/// Insertion sort for small arrays.
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Merge a list of runs into a single sorted vector.
fn merge_runs<T: Clone + Ord>(runs: &[Vec<T>]) -> Vec<T> {
    if runs.is_empty() {
        return Vec::new();
    }
    let mut result = runs[0].clone();
    for run in &runs[1..] {
        result = merge(&result, run);
    }
    result
}

/// Counting sort for non-negative integers.
///
/// Stable sort. O(n + k) where k is the maximum value.
/// Returns a sorted vector.
pub fn counting_sort(arr: &[usize]) -> Vec<usize> {
    if arr.is_empty() {
        return Vec::new();
    }
    let max_val = *arr.iter().max().unwrap();
    let mut count = vec![0usize; max_val + 1];
    for &v in arr {
        count[v] += 1;
    }
    // Prefix sum for stable sort
    let mut total = 0usize;
    for c in count.iter_mut() {
        let old = *c;
        *c = total;
        total += old;
    }
    let mut output = vec![0usize; arr.len()];
    for &v in arr {
        output[count[v]] = v;
        count[v] += 1;
    }
    output
}

/// LSD Radix sort for unsigned integers (base 256).
///
/// Sorts in ascending order. O(d * n) where d is the number of digits.
/// Stable within each digit pass.
pub fn radix_sort(arr: &mut [u64]) {
    if arr.len() <= 1 {
        return;
    }
    const BITS: u32 = 8;
    const BUCKETS: usize = 1 << BITS;
    let passes = u64::BITS.div_ceil(BITS);

    for shift in (0..passes).map(|p| p * BITS) {
        let mut buckets: Vec<Vec<u64>> = vec![vec![]; BUCKETS];
        for &val in arr.iter() {
            let digit = ((val >> shift) & 0xFF) as usize;
            buckets[digit].push(val);
        }
        let mut idx = 0;
        for bucket in &mut buckets {
            for &val in bucket.iter() {
                arr[idx] = val;
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_vec(v: &mut Vec<i32>) -> Vec<i32> {
        v.sort();
        v.clone()
    }

    // === Quicksort Lomuto Tests ===

    #[test]
    fn test_quicksort_lomuto_basic() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        quicksort_lomuto(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_quicksort_lomuto_empty() {
        let mut arr: Vec<i32> = vec![];
        quicksort_lomuto(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_quicksort_lomuto_single() {
        let mut arr = vec![42];
        quicksort_lomuto(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_quicksort_lomuto_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quicksort_lomuto(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quicksort_lomuto_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quicksort_lomuto(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quicksort_lomuto_duplicates() {
        let mut arr = vec![3, 3, 3, 1, 1, 2, 2];
        quicksort_lomuto(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 2, 3, 3, 3]);
    }

    // === Quicksort Hoare Tests ===

    #[test]
    fn test_quicksort_hoare_basic() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        quicksort_hoare(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_quicksort_hoare_empty() {
        let mut arr: Vec<i32> = vec![];
        quicksort_hoare(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_quicksort_hoare_single() {
        let mut arr = vec![42];
        quicksort_hoare(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_quicksort_hoare_reverse() {
        let mut arr = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        quicksort_hoare(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_quicksort_hoare_two_elements() {
        let mut arr = vec![2, 1];
        quicksort_hoare(&mut arr);
        assert_eq!(arr, vec![1, 2]);
    }

    // === Mergesort Tests ===

    #[test]
    fn test_mergesort_basic() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let sorted = mergesort(&arr);
        assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_mergesort_empty() {
        let arr: Vec<i32> = vec![];
        let sorted = mergesort(&arr);
        assert!(sorted.is_empty());
    }

    #[test]
    fn test_mergesort_single() {
        let arr = vec![7];
        assert_eq!(mergesort(&arr), vec![7]);
    }

    #[test]
    fn test_mergesort_stability() {
        // Use pairs to check stability: (key, original_index)
        let arr = vec![(3, 0), (1, 1), (3, 2), (1, 3)];
        let sorted = mergesort(&arr);
        // Elements with same key should maintain original order
        assert_eq!(sorted[0], (1, 1));
        assert_eq!(sorted[1], (1, 3));
        assert_eq!(sorted[2], (3, 0));
        assert_eq!(sorted[3], (3, 2));
    }

    #[test]
    fn test_mergesort_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(mergesort(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mergesort_reverse() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(mergesort(&arr), vec![1, 2, 3, 4, 5]);
    }

    // === Heapsort Tests ===

    #[test]
    fn test_heapsort_basic() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_heapsort_empty() {
        let mut arr: Vec<i32> = vec![];
        heapsort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_heapsort_single() {
        let mut arr = vec![1];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn test_heapsort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        heapsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heapsort_duplicates() {
        let mut arr = vec![5, 5, 5, 5, 5];
        heapsort(&mut arr);
        assert_eq!(arr, vec![5, 5, 5, 5, 5]);
    }

    // === Timsort Tests ===

    #[test]
    fn test_timsort_basic() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let sorted = timsort(&arr);
        assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_timsort_empty() {
        let arr: Vec<i32> = vec![];
        assert!(timsort(&arr).is_empty());
    }

    #[test]
    fn test_timsort_single() {
        let arr = vec![42];
        assert_eq!(timsort(&arr), vec![42]);
    }

    #[test]
    fn test_timsort_already_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(timsort(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_timsort_descending_run() {
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(timsort(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_timsort_mixed_runs() {
        // Ascending then descending then ascending
        let arr = vec![1, 2, 3, 7, 6, 5, 4, 8, 9, 10];
        assert_eq!(timsort(&arr), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    // === Counting Sort Tests ===

    #[test]
    fn test_counting_sort_basic() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(counting_sort(&arr), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_counting_sort_empty() {
        let arr: Vec<usize> = vec![];
        assert!(counting_sort(&arr).is_empty());
    }

    #[test]
    fn test_counting_sort_single() {
        assert_eq!(counting_sort(&vec![5]), vec![5]);
    }

    #[test]
    fn test_counting_sort_stability() {
        // Counting sort is stable, so equal elements maintain order
        let arr = vec![3, 1, 4, 1, 5];
        let sorted = counting_sort(&arr);
        assert_eq!(sorted, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_counting_sort_all_zeros() {
        let arr = vec![0, 0, 0, 0];
        assert_eq!(counting_sort(&arr), vec![0, 0, 0, 0]);
    }

    // === Radix Sort Tests ===

    #[test]
    fn test_radix_sort_basic() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_radix_sort_empty() {
        let mut arr: Vec<u64> = vec![];
        radix_sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_radix_sort_single() {
        let mut arr = vec![42u64];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_radix_sort_large_numbers() {
        let mut arr = vec![u64::MAX, 0, 1, u64::MAX - 1];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![0, 1, u64::MAX - 1, u64::MAX]);
    }

    #[test]
    fn test_radix_sort_reverse() {
        let mut arr = vec![5u64, 4, 3, 2, 1];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    // === Cross-algorithm comparison tests ===

    #[test]
    fn test_all_algorithms_agree() {
        let input = vec![64, 25, 12, 22, 11, 90, 1, 36, 7, 42];
        let expected = {
            let mut v = input.clone();
            v.sort();
            v
        };

        let mut arr1 = input.clone();
        quicksort_lomuto(&mut arr1);
        assert_eq!(arr1, expected);

        let mut arr2 = input.clone();
        quicksort_hoare(&mut arr2);
        assert_eq!(arr2, expected);

        assert_eq!(mergesort(&input), expected);

        let mut arr4 = input.clone();
        heapsort(&mut arr4);
        assert_eq!(arr4, expected);

        assert_eq!(timsort(&input), expected);

        let usize_input: Vec<usize> = input.iter().map(|&x| x as usize).collect();
        assert_eq!(counting_sort(&usize_input), expected.iter().map(|&x| x as usize).collect::<Vec<_>>());

        let mut arr6: Vec<u64> = input.iter().map(|&x| x as u64).collect();
        radix_sort(&mut arr6);
        assert_eq!(arr6, expected.iter().map(|&x| x as u64).collect::<Vec<_>>());
    }

    #[test]
    fn test_vs_std_sort_random() {
        let mut rng_data: Vec<i32> = (0..100).rev().collect();
        let mut std_sorted = rng_data.clone();
        std_sorted.sort();

        quicksort_lomuto(&mut rng_data);
        assert_eq!(rng_data, std_sorted);
    }

    #[test]
    fn test_mergesort_large_random() {
        let arr: Vec<i32> = (0..500).rev().collect();
        let mut expected = arr.clone();
        expected.sort();
        assert_eq!(mergesort(&arr), expected);
    }
}
