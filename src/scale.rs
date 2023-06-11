use std::env;
mod maxarg;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please provide at least one number as an argument.");
        return;
    }

    let mut numbers: Vec<f64> = Vec::new();

    for i in 1..args.len() {
        if let Ok(num) = args[i].parse::<f64>() {
            numbers.push(num);
            maxarg::chkarg(&num);
        } else {
            println!("Error: '{}' is not a valid number.", args[i]);
        }
    }

    let count = numbers.len();

    if count > 0 {
        let sum: f64 = numbers.iter().sum();
        let average = sum / count as f64;
        println!("Average: {:.8}", average);

        let mean = sum / count as f64;
        println!("Mean: {:.8}", mean);

        let median = calculate_median(&mut numbers);
        println!("Median: {:.8}", median);

        let softmax = calculate_softmax(&numbers);
        println!("Softmax: {:?}", softmax);

        let min_value = numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        println!("Lowest value: {:.8}", min_value);

        let max_value = numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        println!("Highest value: {:.8}", max_value);

        let range = max_value - min_value;
        println!("Range: {:.8}", range);

        println!("Sum of all values: {:.8}", sum);
    } else {
        println!("No valid numbers provided.");
    }
}

fn calculate_median(numbers: &mut Vec<f64>) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let count = numbers.len();
    if count % 2 == 0 {
        let middle = count / 2;
        (numbers[middle - 1] + numbers[middle]) / 2.0
    } else {
        numbers[count / 2]
    }
}

fn calculate_softmax(numbers: &[f64]) -> Vec<f64> {
    let exp_sum: f64 = numbers.iter().map(|x| x.exp()).sum();
    numbers.iter().map(|x| x.exp() / exp_sum).collect()
}
