fn main() {
    let mut arr: [i32; 6] = [1, 3, 4, 5, 6, 8];

    let mut i = 0;
    let mut j = arr.len() - 1;

    while i < j {
        if arr[i] % 2 != 0 && arr[j] % 2 == 0 {
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            i += 1;
            j -= 1;
        } else {
            i += 1;
            j -= 1;
        }
    }

    println!("{:?}", arr);
}
