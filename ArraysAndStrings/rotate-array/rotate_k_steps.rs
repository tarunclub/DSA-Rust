fn main() {
    let mut nums: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let k = 3;

    reverse(&mut nums, 0, nums.len() - k - 1);
    reverse(&mut nums, nums.len() - k, nums.len() - 1);
    reverse(&mut nums, 0, nums.len() - 1);

    println!("{:#?}", nums);
}

fn reverse(nums: &mut [i32], mut start: usize, mut end: usize) {
    let mut i = start;
    let mut j = end;

    while i < j {
        let temp = nums[i];
        nums[i] = nums[j];
        nums[j] = temp;
        i += 1;
        j -= 1;
    }
}
