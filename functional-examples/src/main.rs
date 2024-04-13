fn main() {
    let mut sum = 0;

    for i in 1..11 {
        sum += i;
    }

    println!("Sum of 1 to 10 is: {}", sum);

    let sum = (1..11).fold(0, |sum, i| sum + i);

    println!("Sum of 1 to 10 is: {}", sum);
}
