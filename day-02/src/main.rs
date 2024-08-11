fn main() {
    let mut answer: i64 = 0;
    let max_blue: i64 = 14;
    let max_green: i64 = 13;
    let max_red: i64 = 12;

    let mut sum_powers: i64 = 0;

    // open input file
    let file = std::fs::read_to_string("input.txt").unwrap();

    // split input file by lines
    let lines: Vec<&str> = file.trim().split("\n").collect();

    // iterate over lines
    for line in lines{
        let mut valid: bool = true;
        //split the line by :
        let parts: Vec<&str> = line.split(":").collect();

        // get the game number from first part by keeping only digits
        let game_num: i64 = parts[0].chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap();

        //split the second part by ;
        let draws: Vec<&str> = parts[1].split(";").collect();

        let mut min_blue: i64 = 0;
        let mut min_green: i64 = 0;
        let mut min_red: i64 = 0;

        // iterate over draws
        for draw in draws{
            //split the draw by ,
            let draw_parts: Vec<&str> = draw.split(",").collect();

            //iterate over draw parts
            for color in draw_parts{
                let count: i64 = color.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap();

                if color.contains("blue"){
                    if count > max_blue{
                        valid = false;
                    }

                    //keep max blue count in min_blue
                    if count > min_blue{
                        min_blue = count;
                    }
                }
                if color.contains("red"){
                    if count > max_red{
                        valid = false;
                    }

                    //keep max red count in min_red
                    if count > min_red{
                        min_red = count;
                    }
                }
                if color.contains("green"){
                    if count > max_green{
                        valid = false;
                    }

                    //keep max green count in min_green
                    if count > min_green{
                        min_green = count;
                    }
                }
            }
        }

        //calculate sum of min colors
        sum_powers += min_blue * min_green * min_red;


        if valid {
            answer += game_num;
        }

    }

    println!("{}", answer);
    println!("{}", sum_powers);
}
