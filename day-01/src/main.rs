fn main() {
    let mut answer: i64 = 0;

    // open input file
    let file = std::fs::read_to_string("input.txt").unwrap();

    // split input file by lines
    let lines: Vec<&str> = file.trim().split("\n").collect();

    // iterate over lines
    for line in lines{
        println!("{}", line);


        let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let replacements = ["o1e", "t2o", "t333e", "f44r", "f55e", "s6x", "s777n", "e888t", "n99e"];


        //replace numbers with the replacements
        let mut line = line.to_string();
        for i in 0..9{
            line = line.replace(numbers[i], replacements[i]);
        }
        
        println!("{}", line);
        //remove the non-digit cahracters from the line
        let cleaned_line = line.chars().filter(|c| c.is_digit(10)).collect::<String>();

        println!("{}", cleaned_line);

        let first_char = cleaned_line.chars().nth(0).unwrap();
        let last_char = cleaned_line.chars().nth(cleaned_line.len()-1).unwrap();

        //convert chars to int64
        let first_num: i64 = first_char.to_string().parse().unwrap();
        let last_num: i64 = last_char.to_string().parse().unwrap();
        println!("{} + {}{}", answer,first_num, last_num);
        answer += first_num*10 + last_num;
    }

    println!("Answer: {}", answer);


}
