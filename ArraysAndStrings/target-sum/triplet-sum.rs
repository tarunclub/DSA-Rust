fn main() {
    let arr: [i32; 5] = [1, 4, 5, 6, 3];
    let target = 12;

    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            for k in (j + 1)..arr.len() {
                if arr[i] + arr[j] + arr[k] == target {
                    println!("{i}, {j}, {k}");
                }
            }
        }
    }
}
