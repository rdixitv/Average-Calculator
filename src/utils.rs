use itertools::izip;

pub fn calculate_avg(nums: Vec<f32>, denominators: &Vec<i32>, outof: i32) -> Vec<f32> {
    let avg = nums.iter().sum::<f32>() / nums.len() as f32;

    if denominators.is_empty() {
        return vec![avg];
    }

    let res = denominators
        .iter()
        .map(|&d| (avg / outof as f32) * d as f32)
        .collect();

    res
}

pub fn print_averages(averages: Vec<f32>, denominators: Vec<i32>) {
    let mut res = String::new();
    for (avg, den) in izip!(averages, denominators) {
        res = format!("{}{}/{} = ", res, avg, den);
    }

    res = String::from(&res[0..(res.len() - 2)]);
    println!("{}", res);
}