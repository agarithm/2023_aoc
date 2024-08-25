
#[derive(Debug, Clone, Copy)]
struct RaceRecord {
    time: i64,
    distance: i64,
}

#[derive(Debug, Clone, Copy)]
struct WinningStrategy {
    time: i64,
    hold_time: i64,
    speed: i64,
    distance: i64,
}

fn calc_distance(time: i64, hold_time: i64) -> i64 {
    let mut distance = 0;
    let run_time = time-hold_time;
    let speed = hold_time;

    distance = run_time * speed;
    return distance;
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = file.lines();
    //line 1 is the RaceRecord.time : "Time: 7 15   30"
    let line1 = lines.next().unwrap();
    let mut time_iter = line1.split_whitespace().skip(1);

    //line 2 is the RaceRecord.distance : "Distance: 100 200 400"
    let line2 = lines.next().unwrap();
    let mut distance_iter = line2.split_whitespace().skip(1);

    //build the RaceRecords
    let mut records: [RaceRecord; 10] = [RaceRecord{time: 0, distance: 0}; 10];
    let mut i = 0;
    for (time, distance) in time_iter.zip(distance_iter) {
        records[i] = RaceRecord{time: time.parse::<i64>().unwrap(), distance: distance.parse::<i64>().unwrap()};
        i += 1;
    }


    //Array of vectorys to hold winning strategies for each race
    let mut strategies: [Vec<WinningStrategy>; 10] = Default::default();

    for r in 0..10 {
        let race = records[r];

        if race.time == 0 {
            continue;
        } else {
            for hold_time in 0..race.time {
                let distance = calc_distance(race.time, hold_time);
                if distance > race.distance {
                    strategies[r].push(WinningStrategy{time: race.time, hold_time: hold_time, speed: hold_time, distance: distance});
                }
            }
        }
    }

    let mut product_1 = 1;
    for (i, strat) in strategies.iter().enumerate() {
        let count = strat.len();
        if(count == 0) {
            continue;
        }
        product_1 *= count;
        println!("Race {}: {}", i, count);
    }
    println!("Total Winning Strategies: {}", product_1);

   //Problem #2: strip all non-numeric characters from the input string treat as a single number
   let time = line1.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<i64>().unwrap();
   let distance = line2.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<i64>().unwrap();
   println!("Time: {}", time);
   println!("Distance: {}", distance);

   //find the shortes hold time that will win the race
   let mut shortest_hold_time = time;
   for hold_time in 0..time {
       let test = calc_distance(time, hold_time);
       if test > distance {
           shortest_hold_time = hold_time;
           println!("Shortest Winning Hold Time: {}", hold_time);
           break;
       }
   }

   //Find the longest hold time
   let mut longest_hold_time = 0;
   for hold_time in (0..=time).rev() {
       let test = calc_distance(time, hold_time);
       if test > distance {
           longest_hold_time = hold_time;
           println!("Longest Winning Hold Time: {}", hold_time);
           break;
       }
   }

   let all_strategies = (longest_hold_time - shortest_hold_time) + 1;
   println!("All Strategies: {}", all_strategies);
}
