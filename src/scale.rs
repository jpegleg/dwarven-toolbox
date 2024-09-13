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

        let median = calculate_median(&mut numbers);
        println!("Median: {:.8}", median);

        let min_value = numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        println!("Lowest value: {:.8}", min_value);

        let max_value = numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        println!("Highest value: {:.8}", max_value);

        let range = max_value - min_value;
        println!("Range: {:.8}", range);

        println!("Sum of all values: {:.8}", sum);
        
        let softmax = calculate_softmax(&numbers);
        let softmax_normalized = normalize_softmax(&softmax);
        println!("Softmax: {:?}", softmax_normalized);

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

fn calculate_softmax(numbers: &[f64] -> Vec<f64> {
    if numbers.is_empty() {
        return Vec::new();
    }
    let max_value = numbers
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let mut exp_sum = 0.0;
    let exp_values: Vec<f64> = numbers
        .iter()
        .map(|&x| {
            let exp_value = (x - max_value).exp();
            exp_sum += exp_value;
            exp_value
        })
        .collect();
    exp_values.iter().map(|&x| x / exp_sum).collect()
}

fn normalize_softmax(softmax: &[f64]) -> Vec<f64> {
    let max_val = *softmax.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let exp_sum: f64 = softmax.iter().map(|x| (x - max_val).exp()).sum();
    softmax.iter().map(|x| (x - max_val).exp() / exp_sum).collect()
}
