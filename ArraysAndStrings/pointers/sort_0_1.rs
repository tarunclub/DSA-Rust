fn main() {
    let mut arr: [i32; 8] = [1, 0, 0, 1, 0, 1, 1, 0];

    let mut i = 0;
    let mut j = arr.len() - 1;

    while i < j {
        if arr[i] == arr[j] {
            j -= 1;
        } else if arr[j] == 0 && arr[i] == 1 {
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            i += 1;
            j -= 1;
        } else {
            i += 1;
        }
    }

    println!("{:?}", arr);
}
