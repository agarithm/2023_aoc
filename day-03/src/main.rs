
#[derive(Debug, Clone, Copy)]
struct Pos{
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Copy)]
struct MarkerVector {
    pos: Pos,
    char: char,
}


#[derive(Debug, Clone, Copy)]
struct NumberVector {
    pos: Pos,
    number: i32,
    next: usize,
    bordering: [Pos; 300],
}


fn main() {
    
    // open input file
    let file = std::fs::read_to_string("input.txt").unwrap();


    //Treat the file as a grid of characters: 
    // - find the width of the file in columns
    // - find the height of the file in rows
    // - treat the '.' character as an empty space
    // - treat contiguous numeric characters on a row as a single number
    // - keep a list of all bordering cells to each number so that we can lookup the neighboring numbers for a given cell
    // - any other character is a marker for an item
    // - list all numbers that are next to an item marker
    // - keep track of markers and their positions and the numbers next to them

    let mut markers: Vec<MarkerVector> = Vec::new();
    let mut numbers: Vec<NumberVector> = Vec::new();
    let mut row = 0;
    let mut col = 0;
    let mut number = 0;
    let mut num_rec: NumberVector = NumberVector{pos: Pos{row: 0, col: 0}, number: 0, next: 0, bordering: [Pos{row: -1, col: -1}; 300]};

    for line in file.lines() {
        for c in line.chars() {
            if c.is_numeric() {
                if number == 0 {
                    //This is the first digit of a number, start a new numberVec
                    num_rec = NumberVector{pos: Pos{row: row, col: col}, number: 0, next: 0, bordering: [Pos{row: -1, col: -1}; 300]};
                }
                num_rec.number = num_rec.number * 10 + c.to_digit(10).unwrap() as i32;
                //add all bordering cells of row,col to the numberVec.bordering list
                // top-left, top, top-right, left, right, bottom-left, bottom, bottom-right
                num_rec.bordering[num_rec.next] = Pos{row: row, col: col}; num_rec.next += 1;
                num_rec.bordering[num_rec.next] = Pos{row: row, col: col-1}; num_rec.next += 1;
                num_rec.bordering[num_rec.next] = Pos{row: row, col: col+1}; num_rec.next += 1;
                num_rec.bordering[num_rec.next] = Pos{row: row-1, col: col}; num_rec.next += 1;
                num_rec.bordering[num_rec.next] = Pos{row: row-1, col: col-1}; num_rec.next += 1;
                num_rec.bordering[num_rec.next] = Pos{row: row-1, col: col+1}; num_rec.next += 1;
                num_rec.bordering[num_rec.next] = Pos{row: row+1, col: col}; num_rec.next += 1;
                num_rec.bordering[num_rec.next] = Pos{row: row+1, col: col-1}; num_rec.next += 1;
                num_rec.bordering[num_rec.next] = Pos{row: row+1, col: col+1}; num_rec.next += 1;
                number = num_rec.number;

            } else {
                if number > 0 {
                    numbers.push(num_rec);
                }
                if c != '.' {
                    markers.push(MarkerVector{pos: Pos{row: row, col: col}, char: c});
                }
                number = 0;
            }
            col += 1;
        }
        if number > 0 {
            //Numbers that are at the end of a row
            numbers.push(num_rec);
            number = 0;
        }
        row += 1;
        col = 0;
    }


    //We have a vector of all the markers and their positions
    //
    //We have a vector of all the numbers and their bordering cells
    //
    //For each marker find the bordering numbers and sum them up
    
    let mut sum = 0;
    for m in markers.iter() {
        println!("Marker: {:?}", m);
        for n in numbers.iter() {
            for b in n.bordering.iter() {
                if b.row == m.pos.row && b.col == m.pos.col {
                    println!("Number: {}", n.number);
                    println!("Border: {:?}", b);
                    sum += n.number;
                    //Break out of the loop, only add the number once
                    break;
                }
            }
        }
    }
    
    println!("Sum of all numbers next to markers: {}", sum);


    //Gear ratios: find all the markers that are '*' and have excatly two numbers next to them
    // - find the product of the two numbers
    // - find the sum of all the products

    let mut product_sum = 0;
    for m in markers.iter() {
        if m.char == '*' {
            let mut num_count = 0;
            let mut product = 1;
            for n in numbers.iter() {
                for b in n.bordering.iter() {
                    if b.row == m.pos.row && b.col == m.pos.col {
                        num_count += 1;
                        product *= n.number;
                        break;
                    }
                }
            }
            if num_count == 2 {
                product_sum += product;
            }
        }
    }
    println!("Sum of all products of markers with two numbers next to them: {}", product_sum);

}
