/// В качестве аргумента принимает ссылку на целочисленный вектор. Сортирует сам вектор по неубыванию. Ничего не возвращает.
/// # Пример
/// ```
/// let mut vector = vec![1, 9, 3, 2, 10, 8];
/// bubble_sort(&mut vector);
/// asserting_eq!(vector, vec![1, 2, 3, 8, 9, 10]);
/// ```
fn bubble_sort(vec: &mut Vec<i32>) {
    let len = vec.len();
    for i in 0..len {
        for j in (i + 1)..len {
            if vec[j] < vec[i] {
                vec.swap(j, i);
            }
        }
    }
}

/// В качестве аргументов принимает две ссылки ***отсортированных*** по неубыванию векторов. Возвращает объёдинённый
/// отсортированный по неубыванию вектор.
///
/// # Пример
///
/// ```
/// let vec1 = vec![1, 3, 4, 6];
/// let vec2 = vec![2, 5, 7];
/// assert_eq!(merge_two_vectors(&vec1, &vec2), vec![1, 2, 3, 4, 5, 6, 7]);
/// ```
fn merge_two_vectors(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    let mut iter1 = 0;
    let mut iter2 = 0;
    let len1 = vec1.len();
    let len2 = vec2.len();
    let mut result_vector: Vec<i32> = Vec::new();

    while iter1 < len1 || iter2 < len2 {
        if iter1 == len1 || iter2 != len2 && vec2[iter2] < vec1[iter1] {
            result_vector.push(vec2[iter2]);
            iter2 += 1;
        } else {
            result_vector.push(vec1[iter1]);
            iter1 += 1;
        }
    }

    return result_vector;
}

/// В качестве аргументов принимает ссылку на целочисленный вектор. Возвращает отсортированный по неубыванию вектор.
///
/// # Пример
///
/// ```
/// let vector = vec![5, 3, 7, 6, 4, 1, 0];
/// assert_eq!(merge_sort(&vector), vec![0, 1, 3, 4, 5, 6, 7]);
/// ```
fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        return vec.to_vec();
    }
    let len_vec = vec.len();
    let left_vector = vec[0..(len_vec / 2)].to_vec();
    let right_vector = vec[(len_vec / 2)..].to_vec();

    return merge_two_vectors(&merge_sort(&left_vector), &merge_sort(&right_vector));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut test_vec: Vec<i32> = Vec::new();
        for i in (0..1000).rev() {
            test_vec.push(i);
        }
        bubble_sort(&mut test_vec);
    }

    #[test]
    fn test_merge_vectors() {
        let vec1 = vec![1, 3, 5, 8, 9];
        let vec2 = vec![2, 4, 6, 10];
        let result = merge_two_vectors(&vec1, &vec2);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 8, 9, 10]);
        assert_eq!(merge_two_vectors(&vec![1], &vec![2]), vec![1, 2]);
    }

    #[test]
    fn test_merge_sort() {
        let vec = vec![10, 4, 2, 6, 7, 3, 6, 10];
        assert_eq!(merge_sort(&vec), [2, 3, 4, 6, 6, 7, 10, 10]);
    }

    #[test]
    fn test_merge() {
        let mut test_vec: Vec<i32> = Vec::new();
        for i in (0..10000000).rev() {
            test_vec.push(i);
        }
        let result = merge_sort(&test_vec.clone());
        test_vec.reverse();
        assert_eq!(result, test_vec);
    }
}