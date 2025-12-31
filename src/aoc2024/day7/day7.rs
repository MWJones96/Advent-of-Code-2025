fn is_valid(nums: &Vec<u64>, idx: usize, int_num: u64, target_num: u64) -> bool {
    if idx == nums.len() {
        return int_num == target_num;
    }
    if int_num > target_num {
        return false;
    }

    is_valid(nums, idx + 1, int_num * nums[idx], target_num)
        || is_valid(nums, idx + 1, int_num + nums[idx], target_num)
}

fn part1() {
    let input = include_str!("input.txt");
    let mut sum: u64 = 0;
    for line in input.lines() {
        let (target, nums) = line.split_once(":").unwrap();
        let target = target.parse::<u64>().unwrap();
        let nums = nums.trim();
        let nums: Vec<u64> = nums.split(" ").map(|n| n.parse::<u64>().unwrap()).collect();
        if is_valid(&nums, 1, nums[0], target) {
            sum += target;
        }
    }

    println!("(Part 1) Total calibration result: {}", sum);
}

fn part2() {
    let _input = include_str!("input.txt");
    println!("(2024 Day 7) Part 2 not implemented yet");
}

pub fn day7() {
    println!("---- 2024 DAY 7 ----");
    part1();
    part2();
    println!("");
}
