fn main() {
    let mut arr: [i32; 5] = [0, 1, 0, 3, 12];
    for i in 0..arr.len() - 1 {
        let mut j = i + 1;
        if arr[i] == 0 && arr[j] != 0 {
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            j += 1;
        } else if arr[i] == arr[j] {
            j += 1;
        }
    }

    println!("{:?}", arr);
}
