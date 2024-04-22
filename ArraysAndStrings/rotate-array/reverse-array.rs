fn main() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    let mut i = 0;
    let mut j = arr.len() - 1;

    while i < j {
        let temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        i += 1;
        j -= 1;
    }

    println!("{:#?}", arr);
}
