/// В качестве аргумента принимает ссылку на отсортированный по неубыванию вектор и ссылку на целое
/// число, которое нужно найти. Возвращает индекс искомого числа.
/// # Пример
///
/// ```
/// let vector = vec![1, 4, 6, 10, 11];
/// assert_eq!(binary_search(&vector, &10), 3);
/// ```
fn binary_search(vec: &Vec<i32>, target: &i32) -> i32 {
    let mut low: usize = 0;
    let mut high = vec.len();

    while low <= high {
        let mid = (low + high) / 2;
        if *target== vec[mid] {
            return mid as i32;
        } else if vec[mid] > *target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test_vec = vec![1, 3, 5, 10, 123];
        assert_eq!(binary_search(&test_vec, &10), 3);
    }

    #[test]
    fn test2() {
        let test_vec = vec![1, 3, 5, 10, 123];
        assert_eq!(binary_search(&test_vec, &1), 0);
    }

    #[test]
    fn test3() {
        let test_vec = vec![1, 3, 5, 10, 123];
        assert_eq!(binary_search(&test_vec, &9), -1);
    }

    #[test]
    fn test4() {
        let mut test_vec: Vec<i32> = Vec::new();
        for i in 0..1999 {
            test_vec.push(i);
        }
        assert_eq!(binary_search(&test_vec, &200), 200);
    }
}