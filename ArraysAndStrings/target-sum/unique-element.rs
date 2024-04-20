fn main() {
    let mut arr: [i32; 7] = [1, 2, 3, 4, 2, 1, 3];

    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] == arr[j] {
                arr[i] = -1;
                arr[j] = -1;
            }
        }
    }

    for elem in 0..arr.len() {
        if arr[elem] != -1 {
            println!("{}", arr[elem]);
        }
    }
}
