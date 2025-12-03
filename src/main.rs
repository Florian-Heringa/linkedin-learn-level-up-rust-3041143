fn median(mut a: Vec<f32>) -> Option<f32> {
    // First check if a is empty
    if a.is_empty() {
        return None;
    };

    // Then sort vec (median is the middle value by ordering, so we need the input data sorted)
    a.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let vec_length = a.len();
    let middle = vec_length / 2;

    // Even and odd cases
    if vec_length.is_multiple_of(2) {
        Some((a[middle] + a[middle - 1]) / 2.0)
    } else {
        Some(a[middle])
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
