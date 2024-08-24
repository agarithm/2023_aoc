
#[derive(Debug, Clone, Copy)]
struct Card {
    winning_nums: [i32; 10],
    my_numbers: [i32; 30],
    count: i32,
    points: i32,
}

fn main() {
    //hashmap of cards
    let mut cards: [Card; 1000] = [Card{winning_nums: [0; 10], my_numbers: [0; 30], count: 0, points: 0}; 1000];

    // open input file
    let file = std::fs::read_to_string("input.txt").unwrap();

    //parse each line: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    for (i, line) in file.lines().enumerate() {
        let mut iter = line.split_whitespace();
        let mut card = cards[i];
        let mut j = 0;
        let mut mode = 0;
        // skip first two items of iter
        iter.next();
        iter.next();


        for num in iter {
            if num == "|" {
                j = 0;
                mode = 1;
                continue;
            }
            if mode == 0{
                card.winning_nums[j] = num.parse::<i32>().unwrap();
            } else {
                card.my_numbers[j] = num.parse::<i32>().unwrap();
            }
            j += 1;
        }
        card.count = 1;
        cards[i] = card;
    }

    //Loop using the index 0:999 of cards
    for i in 0..1000 {
        let card = cards[i];
        println!("Card {}: {:?}", i, card);
        println!("{:?}", card);
        //Loop card.count times
        for c in 0..card.count {
            //Check if card has a winning number
            let mut j = i+1;
            for win_num in card.winning_nums.iter() {
                if win_num == &0 {
                    break;
                }
                for num in card.my_numbers.iter() {
                    if num == &0 {
                        break;
                    }
                    if num == win_num {
                        println!("Card {} has a winning number: {}", i, num);
                        if 0 == c {
                            if cards[i].points == 0 {
                                cards[i].points = 1;
                            }else {
                                cards[i].points *= 2;
                            }
                        }
                        cards[j].count += 1;
                        j += 1;
                    }
                }
            }
        }
    }

    //Sum the counts of cards
    let mut sum = 0;
    let mut points = 0;
    for card in cards.iter() {
        sum += card.count;
        points += card.points;
    }
    println!("Points: {}", points);
    println!("Sum: {}", sum);
}
