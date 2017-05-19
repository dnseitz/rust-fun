
// --- Imperative vs. Functional
pub fn imperative_sum() -> u32 {
    let mut sum = 0;
    for i in 0..10000 {
        sum += i;
    }
    sum
}

pub fn functional_sum() -> u32 {
    (0..10000).sum()
}

pub fn imperative_double_vec() -> Vec<u32> {
    let mut vec = vec![0, 1, 2, 3, 4, 5];

    for elem in &mut vec {
        *elem *= 2;
    }
    vec
}

pub fn functional_double_vec() -> Vec<u32> {
    vec![0, 1, 2, 3, 4, 5].into_iter().map(|x| x * 2).collect()
}

pub fn imperative_filter_value() -> Vec<u32> {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let mut filtered = Vec::new();

    for elem in vec {
        if elem < 4 {
            filtered.push(elem);
        }
    }
    filtered
}

pub fn functional_filter_value() -> Vec<u32> {
    vec![1, 2, 3, 4, 5, 6].into_iter().filter(|&x| x < 4).collect()
}

pub fn imperative_filter_double_sum() -> u32 {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut sum = 0;

    for elem in vec {
        if elem < 5 {
            sum += elem * 2;
        }
    }
    sum
}

pub fn functional_filter_double_sum() -> u32 {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    vec.into_iter().filter(|&x| x < 5).map(|x| x * 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imperative_sum() {
        assert_eq!(imperative_sum(), 49995000);
    }

    #[test]
    fn test_functional_sum() {
        assert_eq!(functional_sum(), 49995000);
    }

    #[test]
    fn test_imperative_double_vec() {
        assert_eq!(imperative_double_vec(), vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_functional_double_vec() {
        assert_eq!(functional_double_vec(), vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_imperative_filter_value() {
        assert_eq!(imperative_filter_value(), vec![1, 2, 3]);
    }

    #[test]
    fn test_functional_filter_value() {
        assert_eq!(functional_filter_value(), vec![1, 2, 3]);
    }

    #[test]
    fn test_imperative_filter_double_sum() {
        assert_eq!(imperative_filter_double_sum(), 20);
    }

    #[test]
    fn test_functional_filter_double_sum() {
        assert_eq!(functional_filter_double_sum(), 20);
    }
}
