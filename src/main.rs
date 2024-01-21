mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: avgcalc [nums] -b [outof] (-o [denominators])");
        std::process::exit(1);
    }

    let (mut nums, denominators) = args.split_at(args.iter().position(|i| *i == "-b").unwrap_or(args.len()));

    let (outof, mut denominators) = denominators.split_at(denominators.iter().position(|i| *i == "-o").unwrap_or(denominators.len()));

    if outof.len() > 2 {
        println!("The number that the average is out of must be a single integer.\nUsage: avgcalc [nums] -b [outof] (-o [denominators])");
        std::process::exit(1);
    }

    if denominators.len() != 0 {
        denominators = &denominators[1..];
    }

    nums = &nums[1..];

    let nums: Vec<f32> = nums.iter().map(|s| s.parse::<f32>().unwrap()).collect();
    let denominators: Vec<i32> = denominators.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let outof = match outof.len() {
        0 | 1 => 100,
        _ =>  outof[1].parse::<i32>().unwrap(),
    };


    let averages = utils::calculate_avg(nums, &denominators, outof);

    // println!("{:?}", averages);
    utils::print_averages(averages, denominators);
}