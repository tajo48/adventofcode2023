pub fn main() {
    let kek = include_str!("../input.txt");
    println!("{}", sum_and_delete(letter_to_digit(kek.to_string())));
}

fn letter_to_digit(input: String) -> String{
    let numbers = vec!["----", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut output:String = "".to_string();
    let mut creating_line:String="".to_string();
    // function to make eightwothree to 83 and not 823
    for line in input.lines() {
        for chars in line.chars() {
            creating_line = creating_line + &chars.to_string();
            for (index, number) in numbers.iter().enumerate() {
            creating_line = creating_line.replace(number, &index.to_string());
            }
        }
    output = output + &creating_line + "\n";
    creating_line = "".to_string(); 
    println!("{}", output);
    }

    return output 
}

fn sum_and_delete(kek: String) -> u32{
    let mut sum = 0;
    for line in kek.lines() {
        let num:u32 = (line.replace(|c: char| c.is_alphabetic(), "")).parse().unwrap();
        let first_digit:u32 = num.to_string().chars().next().unwrap().to_digit(10).unwrap();
        let last_digit = num.to_string().chars().last().unwrap().to_digit(10).unwrap();
        sum += first_digit* 10 + last_digit; 
    }
    return sum 
}
