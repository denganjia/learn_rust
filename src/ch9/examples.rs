pub fn std_examples() {
    let mut vec1 = vec![1, 2, 3, 4];
    let arr = [1, 2, 3, 4];

    println!("2 in vec1:{}", vec1.iter().any(|x| *x == 2));
    println!("2 in arr:{}", arr.iter().any(|&x| x == 2));

    println!(
        "Find 2 in vec1:{:?}",
        vec1.iter_mut().find(|&&mut x| x == 2)
    );

    println!("Find 2 in arr:{:?}", arr.into_iter().find(|&x| x == 2))
}
