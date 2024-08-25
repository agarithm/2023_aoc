

#[derive(Debug, Clone, Copy)]
struct MapValue {
    src_start: i64,
    src_end: i64,
    dest_start: i64,
    dest_end: i64,
}

#[derive(Debug, Clone, Copy)]
struct SeedRange {
    src_start: i64,
    src_end: i64,
}


fn lookup(map: &Vec<MapValue>, src: i64) -> i64 {
    for m in map.iter() {
        if src >= m.src_start && src <= m.src_end {
            let offset = src - m.src_start;
            println!("src: {}, src_start: {}, offset: {}, dest_start: {}, dest: {}", src, m.src_start, offset, m.dest_start, m.dest_start + offset);
            return m.dest_start + offset;
        }
    }
    return src;
}

fn reverse_lookup(map: &Vec<MapValue>, dest: i64) -> i64 {
    for m in map.iter() {
        if dest >= m.dest_start && dest <= m.dest_end {
            let offset = dest - m.dest_start;
            println!("dest: {}, dest_start: {}, offset: {}, src_start: {}, src: {}", dest, m.dest_start, offset, m.src_start, m.src_start + offset);
            return m.src_start + offset;
        }
    }
    return dest;
}


fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut seeds: Vec<i64> = Vec::new();
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    let mut seed_to_soil: Vec<MapValue> = Vec::new();
    let mut soil_to_fert: Vec<MapValue> = Vec::new();
    let mut fert_to_water: Vec<MapValue> = Vec::new();
    let mut water_to_light: Vec<MapValue> = Vec::new();
    let mut light_to_temp: Vec<MapValue> = Vec::new(); 
    let mut temp_to_humidity: Vec<MapValue> = Vec::new();
    let mut humidity_to_loc: Vec<MapValue> = Vec::new();

    //First line is the seeds list: "seeds: 1 2 3 4 5 6 7 8 9 10"
    let mut iter = file.lines();
    let seeds_line = iter.next().unwrap();
    for seed in seeds_line.split_whitespace().skip(1) {
        seeds.push(seed.parse::<i64>().unwrap());
    }

    let mut seed_line_iter = seeds_line.split_whitespace().skip(1);
    //pull teo items at a time from the seed line and parse as start and count
    while let Some(start) = seed_line_iter.next() {
        let count = seed_line_iter.next().unwrap();
        let seed_range = SeedRange{src_start: start.parse::<i64>().unwrap(), src_end: start.parse::<i64>().unwrap() + count.parse::<i64>().unwrap()};
        seed_ranges.push(seed_range);
    }

    println!("{:?}", seeds);
    //Skip the empty line
    iter.next();

    iter.next();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        println!("{}", line);
        let mut iter = line.split_whitespace();
        let dest_start = iter.next().unwrap().parse::<i64>().unwrap();
        let src_start = iter.next().unwrap().parse::<i64>().unwrap();
        let count = iter.next().unwrap().parse::<i64>().unwrap();
        let src_end = src_start + count - 1;
        let dest_end = dest_start + count - 1;
        let map = MapValue{src_start: src_start, src_end: src_end, dest_start: dest_start, dest_end: dest_end};
        println!("seed_to_soil: {:?}", map);
        seed_to_soil.push(map);
    }
    
    //When we encounter an empty line transistion to the next map
    iter.next();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        println!("{}", line);
        let mut iter = line.split_whitespace();
        let dest_start = iter.next().unwrap().parse::<i64>().unwrap();
        let src_start = iter.next().unwrap().parse::<i64>().unwrap();
        let count = iter.next().unwrap().parse::<i64>().unwrap();
        let src_end = src_start + count - 1;
        let dest_end = dest_start + count - 1;
        let map = MapValue{src_start: src_start, src_end: src_end, dest_start: dest_start, dest_end: dest_end};
        println!("soil_to_fert: {:?}", map);
        soil_to_fert.push(map);
    }

    iter.next();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        println!("{}", line);
        let mut iter = line.split_whitespace();
        let dest_start = iter.next().unwrap().parse::<i64>().unwrap();
        let src_start = iter.next().unwrap().parse::<i64>().unwrap();
        let count = iter.next().unwrap().parse::<i64>().unwrap();
        let src_end = src_start + count - 1;
        let dest_end = dest_start + count - 1;
        let map = MapValue{src_start: src_start, src_end: src_end, dest_start: dest_start, dest_end: dest_end};
        println!("fert_to_water: {:?}", map);
        fert_to_water.push(map);
    }

    iter.next();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        println!("{}", line);
        let mut iter = line.split_whitespace();
        let dest_start = iter.next().unwrap().parse::<i64>().unwrap();
        let src_start = iter.next().unwrap().parse::<i64>().unwrap();
        let count = iter.next().unwrap().parse::<i64>().unwrap();
        let src_end = src_start + count - 1;
        let dest_end = dest_start + count - 1;
        let map = MapValue{src_start: src_start, src_end: src_end, dest_start: dest_start, dest_end: dest_end};
        println!("water_to_light: {:?}", map);
        water_to_light.push(map);
    }

    iter.next();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        println!("{}", line);
        let mut iter = line.split_whitespace();
        let dest_start = iter.next().unwrap().parse::<i64>().unwrap();
        let src_start = iter.next().unwrap().parse::<i64>().unwrap();
        let count = iter.next().unwrap().parse::<i64>().unwrap();
        let src_end = src_start + count - 1;
        let dest_end = dest_start + count - 1;
        let map = MapValue{src_start: src_start, src_end: src_end, dest_start: dest_start, dest_end: dest_end};
        println!("light_to_temp: {:?}", map);
        light_to_temp.push(map);
    }

    iter.next();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        println!("{}", line);
        let mut iter = line.split_whitespace();
        let dest_start = iter.next().unwrap().parse::<i64>().unwrap();
        let src_start = iter.next().unwrap().parse::<i64>().unwrap();
        let count = iter.next().unwrap().parse::<i64>().unwrap();
        let src_end = src_start + count - 1;
        let dest_end = dest_start + count - 1;
        let map = MapValue{src_start: src_start, src_end: src_end, dest_start: dest_start, dest_end: dest_end};
        println!("temp_to_humidity: {:?}", map);
        temp_to_humidity.push(map);
    }

    iter.next();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        println!("{}", line);
        let mut iter = line.split_whitespace();
        let dest_start = iter.next().unwrap().parse::<i64>().unwrap();
        let src_start = iter.next().unwrap().parse::<i64>().unwrap();
        let count = iter.next().unwrap().parse::<i64>().unwrap();
        let src_end = src_start + count - 1;
        let dest_end = dest_start + count - 1;
        let map = MapValue{src_start: src_start, src_end: src_end, dest_start: dest_start, dest_end: dest_end};
        println!("humidity_to_loc: {:?}", map);
        humidity_to_loc.push(map);
    }


    //print all the locations
    let mut min_loc = -1;
    for seed in seeds.iter() {
        let soil = lookup(&seed_to_soil, *seed);
        let fert = lookup(&soil_to_fert, soil);
        let water = lookup(&fert_to_water, fert);
        let light = lookup(&water_to_light, water);
        let temp = lookup(&light_to_temp, light);
        let humidity = lookup(&temp_to_humidity, temp);
        let loc = lookup(&humidity_to_loc, humidity);
        if min_loc == -1 || loc < min_loc {
            min_loc = loc;
        }
        println!("Seed: {}, Soil: {}, Fert: {}, Water: {}, Light: {}, Temp: {}, Humidity: {}, Loc: {}", seed, soil, fert, water, light, temp, humidity, loc);
    }
    println!("Problem #1 = Min Loc: {}", min_loc);


    //Problem #2: It's diffult to go from Seed to Location as we need to compute all the seed and
    //then find the minimum location.  Instead we'll start from Location = 0 and go backwards to
    //find if any seed can be used to get to Location = 0, then increment the location until we
    //find a seed that can get us to the new location.  First seed found is the minimum location.
    let mut max_location = 0;
    for loc in humidity_to_loc.iter() {
        if loc.dest_end > max_location {
            max_location = loc.dest_end;
        }
    }   

    min_loc = -1;
    let mut min_seed = -1;
    for loc in 0..max_location {
        if min_seed != -1 {
            break;
        }
        let humidity = reverse_lookup(&humidity_to_loc, loc);
        let temp = reverse_lookup(&temp_to_humidity, humidity);
        let light = reverse_lookup(&light_to_temp, temp);
        let water = reverse_lookup(&water_to_light, light);
        let fert = reverse_lookup(&fert_to_water, water);
        let soil = reverse_lookup(&soil_to_fert, fert);
        let seed = reverse_lookup(&seed_to_soil, soil);
        for seed_range in seed_ranges.iter() {
            if seed >= seed_range.src_start && seed <= seed_range.src_end {
                min_seed = seed;
                min_loc = loc;
            }
        }
    }
    println!("Problem #2 = Min Loc: {}", min_loc);
}
