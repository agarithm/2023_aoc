use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Link {
    id: String,
    left: String,
    right: String,
    first_z: usize,
    next_z: usize,
}



fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = file.lines();
    let directions = lines.next().unwrap();
    let mut lookup: HashMap<String, Link> = HashMap::new();

    for line in lines.skip(1) {
        //Typical line: AAA = (BBB, CCC)
        //Parse the line as "id = (left, right)"
        let mut iter = line.split_whitespace();
        let id = iter.next().unwrap();
        let _ = iter.next().unwrap(); //Skip the "="
        //Keep only the alphanumeric characters
        let left = iter.next().unwrap().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
        let right = iter.next().unwrap().chars().filter(|c| c.is_alphanumeric()).collect::<String>();

        //store the parse values in a hashmap
        let link = Link {
            id: id.to_string(),
            left: left,
            right: right,
            first_z: 0,
            next_z: 0,
        };

        lookup.insert(id.to_string(), link);
    }

    //Print the hashmap
    for (key, value) in lookup.iter() {
        println!("{}: {:?}", key, value);
    }

    if lookup.contains_key("AAA") && lookup.contains_key("ZZZ") {
        //Find the shortest path to ZZZ

        let mut steps = 0;
        let mut current = lookup.get("AAA").unwrap();
        loop {
            let next = steps % directions.len();
            let choice = directions.chars().nth(next).unwrap();
            steps += 1;
            let next_id = match choice {
                'L' => lookup.get(&current.id).unwrap().left.clone(),
                'R' => lookup.get(&current.id).unwrap().right.clone(),
                _ => panic!("Invalid direction"),
            };
            if next_id == "ZZZ" {
                break;
            }
            current = lookup.get(&next_id).unwrap();
        }
        println!("Part 1 Steps: {}", steps);
    }

    //Find all the ids that end in ??A
    //keep them in an array

    let mut starting_links: Vec<Link> = Vec::new();
    let mut middle_links: Vec<Link> = Vec::new();
    let mut ending_links: Vec<Link> = Vec::new();
    for (_key, value) in lookup.iter() {
        if value.id.ends_with("A") {
            starting_links.push(value.clone());
        }
    }
    println!("Starting Links: {:?}", starting_links);


    for link in starting_links.iter_mut() {
        let mut steps = 0;
        let mut current = link.clone();
        loop {
            let next = steps % directions.len();
            let choice = directions.chars().nth(next).unwrap();
            steps += 1;
            let next_id = match choice {
                'L' => lookup.get(&current.id).unwrap().left.clone(),
                'R' => lookup.get(&current.id).unwrap().right.clone(),
                _ => panic!("Invalid direction"),
            };
            let next_node = lookup.get(&next_id).unwrap().clone();
            current = next_node;
            if next_id.ends_with("Z") {
                break;
            }
        }
        link.first_z = steps;

        loop {
            let next = steps % directions.len();
            let choice = directions.chars().nth(next).unwrap();
            steps += 1;
            let next_id = match choice {
                'L' => lookup.get(&current.id).unwrap().left.clone(),
                'R' => lookup.get(&current.id).unwrap().right.clone(),
                _ => panic!("Invalid direction"),
            };
            let next_node = lookup.get(&next_id).unwrap().clone();
            current = next_node;
            if next_id.ends_with("Z") {
                break;
            }
        }
        link.next_z = steps;
        ending_links.push(link.clone());
    }

    println!("Ending Links: {:?}", ending_links);
    let mut max_steps = 0;
    let mut min_steps = 9999999999;
    for link in ending_links.iter_mut() {
        if link.first_z * 2 < link.next_z {
            println!("Link Starts in Middle: {:?}", link);
        } else {
            println!("Link Starts at Beginning: {:?}", link);
        }
        if link.first_z > max_steps {
            max_steps = link.first_z;
        }
        if link.first_z < min_steps {
            min_steps = link.first_z;
        }
    }

    let mut steps = max_steps; //This is the answer
    println!("Max Steps: {}", max_steps);
    println!("Min Steps: {}", min_steps);
    while true {
        let mut all_on_z = true;
        for link in ending_links.iter_mut() {
            if steps % link.next_z != 0 {
                all_on_z = false;
                break;
            }
        }   
        if all_on_z {
            break;
        }
        steps += max_steps;
    }

    steps /= 2; //Nyquist frequency;

    println!("Part 2 Steps: {}", steps);

}
