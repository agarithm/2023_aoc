


#[derive(Debug, Clone)]
struct SubSequence {
    line: String,
    prev_number: i64,
    next_number: i64
}

#[derive(Debug, Clone)]
struct Sequence {
    line: String,
    subsequence: Vec<SubSequence>,
    prev_number: i64,
    next_number: i64
}

fn is_zeros(line: String) -> bool {
    //Is the line a string of 0's separated by spaces?
    let mut is_zeros = true;

    //split the line by whitespace
    let iter = line.split_whitespace();
    for s in iter {
        if s != "0" {
            is_zeros = false;
            break;
        }
    }
    return is_zeros;
}

fn get_last_number(line: String) -> i64 {
    //split the line by whitespace
    let mut numbers = Vec::new();
    for n in line.split_whitespace() {
        let num = n.parse::<i64>().unwrap();
        numbers.push(num);
    }
    return numbers[numbers.len()-1];
}

fn get_prev_number(line: String) -> i64 {
    //split the line by whitespace
    let mut numbers = Vec::new();
    for n in line.split_whitespace() {
        let num = n.parse::<i64>().unwrap();
        numbers.push(num);
    }
    return numbers[0];
}

fn get_next_number(sequence: Sequence, depth: usize) -> Sequence {
    let mut sequence = sequence.clone();
    let subsequence = sequence.subsequence[depth].clone();
    if is_zeros(subsequence.line.clone()) {
        sequence.subsequence[depth].next_number = 0;
        sequence.subsequence[depth].prev_number = 0;
        return sequence.clone();
    } else {
        sequence = get_next_number(sequence, depth+1);
        let next_number = sequence.subsequence[depth+1].next_number;
        let prev_number = sequence.subsequence[depth+1].prev_number;
        sequence.subsequence[depth].next_number = get_last_number(subsequence.line.clone()) + next_number;
        sequence.subsequence[depth].prev_number = get_prev_number(subsequence.line.clone()) - prev_number;
        return sequence.clone();
    }
}

fn get_subsequence(line: &str) -> SubSequence {
    //split the line by whitespace
    let mut numbers = Vec::new();
    let mut diff = Vec::new();
    for n in line.split_whitespace() {
        let num = n.parse::<i64>().unwrap();
        numbers.push(num);
    }
    for i in 0..numbers.len()-1 {
        diff.push(numbers[i+1] - numbers[i]);
    }

    //join the diff vector into a string
    let new_line = diff.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ");

    return SubSequence{line: new_line, next_number: 0, prev_number: 0};
}

fn get_sequence(line: &str) -> Sequence {
    let mut next_line = line.to_string();
    let mut sequence: Sequence = Sequence{line: String::new(), subsequence: Vec::new(), next_number: 0, prev_number: 0};

    sequence.line = next_line.clone();
    while true{
        let subsequence = get_subsequence(next_line.as_str());
        sequence.subsequence.push(subsequence.clone());
        if is_zeros(subsequence.line.clone()) {
            break;
        }
        next_line = subsequence.line.clone();
    }

    return sequence;
}


fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = file.lines();
    let mut sequences = Vec::new();

    for line in lines {
        let mut sequence = get_sequence(line);
        sequences.push(sequence.clone());
    }


    let mut i = 0;
    for sequence_in in sequences.clone() {
        let mut sequence = get_next_number(sequence_in, 0);
        sequence.next_number = get_last_number(sequence.line.clone()) + sequence.subsequence[0].next_number;
        sequence.prev_number = get_prev_number(sequence.line.clone()) - sequence.subsequence[0].prev_number;
        println!("Sequence: {} = {} = {}", sequence.prev_number, sequence.line, sequence.next_number);
        for subsequence in sequence.subsequence.clone() {
            println!("Subsequence: {} = {} = {}", subsequence.prev_number, subsequence.line, subsequence.next_number);
        }
        sequences[i] = sequence.clone();
        i += 1;
    }
    
    //sum all the next_numbers
    let mut sum = 0;
    for sequence in sequences.clone() {
        sum += sequence.next_number;
    }

    println!("Problem 1 Sum: {}", sum);

    //Problem 2 Sum prev_numbers
    let mut sum = 0;
    for sequence in sequences.clone() {
        sum += sequence.prev_number;
    }

    println!("Problem 2 Sum: {}", sum);
}

