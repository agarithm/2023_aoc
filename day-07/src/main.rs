use std::cmp::Ordering;

const CARDS: &str = "123456789TJQKAX";

#[derive(Debug, Clone, Copy)]
struct CardCount {
    c: char,
    count: u32, 
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: String,
    value: u32,
    weight: u32,
    joker_value: u32,
    joker_weight: u32,
    bid: u32,
}

//Sort by Value, then Weight
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value == other.value {
            return self.weight.partial_cmp(&other.weight);
        }
        return self.value.partial_cmp(&other.value);
    }
}

//sort by Value then Weight
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value == other.value {
            return self.weight.cmp(&other.weight);
        }
        return self.value.cmp(&other.value);
    }
}


fn get_weight(cards: &str, jokers: bool) -> u32 {
    let mut weight = String::new();
    let mut cards = cards.to_string();

    if jokers && has_joker(&cards) {
        //Replace the "J" with a "X" for the Joker
        cards = cards.replace("J", "X");
        return get_weight(&cards, jokers);
    }

    for c in cards.chars() {
        match c {
            '1' => weight.push('1'), //Not a card, but here for completeness
            '2' => weight.push('2'),
            '3' => weight.push('3'),
            '4' => weight.push('4'),
            '5' => weight.push('5'),
            '6' => weight.push('6'),
            '7' => weight.push('7'),
            '8' => weight.push('8'),
            '9' => weight.push('9'),
            'T' => weight.push('A'),
            'J' => weight.push('B'),
            'Q' => weight.push('C'),
            'K' => weight.push('D'),
            'A' => weight.push('E'),
            'X' => weight.push('1'), //Joker
            _ => (),
        }
    }
    //Parse the weight as a Hex number
    return u32::from_str_radix(&weight, 16).unwrap();
}

fn has_joker(cards: &str) -> bool {
    return cards.contains("J");
}

fn get_value(cards: &str, jokers: bool) -> u32 {

    if jokers && has_joker(cards) {
        //Find the Max Value by substituting the Joker for each card
        let mut max_value = 0;
        let mut max_card = '1';
        let mut cards = cards.to_string();
        while has_joker(&cards) {
            let card_copy = cards.clone();
            for c in CARDS.chars() {
                //Replace the Joker with the card one at a time
                //skip J and X and 1
                if c == 'J' || c == 'X' || c == '1' {
                    continue;
                }
                let new_cards = card_copy.replace("J", &c.to_string());
                let value = get_value(&new_cards, true);
                if value > max_value {
                    max_value = value;
                    max_card = c;
                }
            }
            cards = cards.replace("J", &max_card.to_string());
        }
        return max_value;
    }

    //calculate Poker Hand Kind
    let mut card_count: Vec<CardCount> = Vec::new();
    for c in CARDS.chars() {
        let count = cards.matches(c).count();
        if count > 0 {
            card_count.push(CardCount{c: c, count: count as u32});
        }
    }

    //Find the Poker hands from the Card Counts
    //Five of a Kind = 7 pts
    //Four of a Kind = 6 pts
    //Full House = 5 pts
    //Three of a Kind = 4 pts
    //Two Pair = 3 pts
    //One Pair = 2 pts
    //High Card = 1 pt
    
    //Five of a Kind
    for c in card_count.iter() {
        if c.count == 5 {
            return 7;
        }
    }

    //Four of a Kind
    for c in card_count.iter() {
        if c.count == 4 {
            return 6;
        }
    }

    //Full House
    let mut three_of_a_kind = false;
    let mut two_of_a_kind = false;
    for c in card_count.iter() {
        if c.count == 3 {
            three_of_a_kind = true;
        }
        if c.count == 2 {
            two_of_a_kind = true;
        }
    }
    if three_of_a_kind && two_of_a_kind {
        return 5;
    }

    //Three of a Kind
    if three_of_a_kind {
        return 4;
    }

    //Two Pair
    let mut pair_count = 0;
    for c in card_count.iter() {
        if c.count == 2 {
            pair_count += 1;
        }
    }
    if pair_count == 2 {
        return 3;
    }

    //One Pair
    if pair_count == 1 {
        return 2;
    }

    //High Card
    return 1;
}


fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines = file.lines();
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let mut iter = line.split_whitespace();
        let cards = iter.next().unwrap();
        let bid = iter.next().unwrap().parse::<u32>().unwrap();
        let value = get_value(&cards,false);
        let weight = get_weight(&cards,false);
        let joker_value = get_value(&cards,true);
        let joker_weight = get_weight(&cards,true);
        hands.push(Hand{cards: cards.to_string(), value: value, weight: weight, joker_value: joker_value, joker_weight: joker_weight, bid: bid});
    }


    hands.sort();
    let mut winnings: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        let mult = i + 1;
        winnings += hand.bid * mult as u32;
    }
    println!("Winnings: {}", winnings);
    
    //Joker winnings
    hands.sort_by(|a, b| a.joker_value.cmp(&b.joker_value).then(a.joker_weight.cmp(&b.joker_weight)));

    winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        let mult = i + 1;
        winnings += hand.bid * mult as u32;
    }
    println!("Joker Winnings: {}", winnings);
}
