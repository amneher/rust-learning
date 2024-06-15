
// fn calculate_interest(rate: f32, amount: f32) -> f32 {
//     let interest: f32 = rate * amount;
//     return interest;
// }

// fn fibonacci(n: i32) -> i32 {
//     if n < 2 {
//         return 1;
//     }
//     else {
//         return fibonacci(n-1) + fibonacci(n-2);
//     }

// }

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    }
    else {
        a
    }
}

fn main() {
    // let interest_rate: f32 = 0.011;
    // let loan_amount: f32 = 4322.24;
    // let accrued_interest: f32 = calculate_interest(interest_rate, loan_amount);
    // println!("Accrued interest: {}", accrued_interest);

//     let fib_total: i32 = fibonacci(20);
//     println!("Fibonacci: {}", fib_total);

    println!("GCD: {}", gcd(243, 57))

}


