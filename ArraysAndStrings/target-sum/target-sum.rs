fn main() {
    let arr: [i32; 6] = [4, 6, 3, 5, 8, 2];
    let target = 7;

    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] + arr[j] == target {
                println!("{i}, {j}");
            }
        }
    }
}
