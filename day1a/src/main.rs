pub fn main() {
    let kek = include_str!("../input.txt");
    println!("{}", sum_and_delete(kek));
}

fn sum_and_delete(kek: &str) -> u32{
    let mut sum = 0;
    for line in kek.lines() {
        let num:u32 = (line.replace(|c: char| c.is_alphabetic(), "")).parse().unwrap();
        let first_digit:u32 = num.to_string().chars().next().unwrap().to_digit(10).unwrap();
        let last_digit = num.to_string().chars().last().unwrap().to_digit(10).unwrap();
        sum += first_digit* 10 + last_digit; 
    }
    return sum 
}
