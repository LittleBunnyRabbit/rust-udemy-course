fn use_slice(slice: &mut[i32]) {
    println!("first el: {}, length = {}", slice[0], slice.len());
    slice[slice.len() - 1] = 4321
}

pub fn slices() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    use_slice(&mut data);

    println!("{:?}", data);
}