fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got value: {val}");
    }

    let v2 = vec![1, 2, 3];
    let v2_mapped: Vec<i32> = v2.iter().map(|x| x * 2).collect();
    assert_eq!(v2_mapped, vec![2, 4, 6]);
}
