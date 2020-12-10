fn is_sum(list: &[u64]) -> bool {
    let num = *list.last().unwrap();
    for i in 0..list.len() - 2 {
        for j in i..list.len() - 1 {
            if list[i] + list[j] == num {
                return true;
            }
        }
    }
    false
}

fn find_first(preamble_len: usize, nums: &Vec<u64>) -> usize {
    let l = preamble_len + 1;
    for i in 0..nums.len() - l {
        //println!("{}-{} {:?} {}", i, i+l, &nums[i..i+l], is_sum(&nums[i..i+l]));
        if !is_sum(&nums[i..i + l]) {
            return i + preamble_len;
        }
    }
    return nums.len();
}

fn find_contiguous_sum(num: u64, nums: &Vec<u64>) -> u64 {
    for i in 0..nums.len() - 1 {
        let mut sum = nums[i];
        //println!("Starting at {} {}", i, sum);
        let mut len = 1;
        let mut min = nums[i];
        let mut max = nums[i];
        while sum < num && len + i < nums.len() {
            min = min.min(nums[i + len]);
            max = max.max(nums[i + len]);
            sum += nums[i + len];
            //println!("  Sum went to {} (+= {})", sum, nums[i+len]);
            len += 1;
        }
        if sum == num {
            //println!("Found exact sum! Min-max {} {}", min, max);
            return min + max;
        }
    }
    return 0;
}

fn main() {
    let nums = include_str!("input")
        .trim()
        .split("\n")
        .map(|s| s.trim().parse::<u64>().expect("Not a u64"))
        .collect::<Vec<_>>();

    let first = find_first(25, &nums);
    println!("First invalid: {}", nums[first]);
    println!(
        "Contiguous sum: {}",
        find_contiguous_sum(nums[first], &nums)
    );
}
