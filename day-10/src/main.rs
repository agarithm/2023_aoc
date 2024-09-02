const MAP_SIZE: usize = 140;

const NE: usize = 0;
const N: usize  = 1;
const NW: usize = 2;
const W: usize  = 3;
const C: usize  = 4;
const E: usize  = 5;
const SW: usize = 6;
const S: usize  = 7;
const SE: usize = 8;

#[derive(Debug, Clone, Copy)]
struct Tile {
    pipe: char,
    x: i32,
    y: i32,
    north: bool,
    south: bool,
    east: bool,
    west: bool,
    dist_from_start: i32,
    inside: bool,
}

fn valid_index(i: i32, tile: Tile) -> bool {
    if tile.x == -1 || tile.y == -1 {
        return false;
    }
    return i >= 0 && i < MAP_SIZE as i32;
}

fn get_tile(x: i32, y: i32) -> Tile {
    unsafe {
        if x >= 0 && x < MAP_SIZE as i32 && y >= 0 && y < MAP_SIZE as i32 {
            return map[x as usize][y as usize];
        }
        return Tile{pipe: 'X', x: -1, y: -1, north: false, south: false, east: false, west: false, dist_from_start: -1, inside: true};
    }
}
fn get_north_tile(tile: Tile) -> Tile {
    unsafe {
        if valid_index(tile.y - 1, tile) {
            return map[tile.x as usize][tile.y as usize - 1];
        }
        return Tile{pipe: 'X', x: -1, y: -1, north: false, south: false, east: false, west: false, dist_from_start: -1, inside: true};
    }
}

fn get_south_tile(tile: Tile) -> Tile {
    unsafe {
        if valid_index(tile.y + 1, tile) {
            return map[tile.x as usize][tile.y as usize + 1];
        }
        return Tile{pipe: 'X', x: -1, y: -1, north: false, south: false, east: false, west: false, dist_from_start: -1, inside: true};
    }
}

fn get_east_tile(tile: Tile) -> Tile {
    unsafe {
        if valid_index(tile.x + 1, tile) {
            return map[tile.x as usize + 1][tile.y as usize];
        }
        return Tile{pipe: 'X', x: -1, y: -1, north: false, south: false, east: false, west: false, dist_from_start: -1, inside: true};
    }
}

fn get_west_tile(tile: Tile) -> Tile {
    unsafe {
        if valid_index(tile.x - 1, tile) {
            return map[tile.x as usize - 1][tile.y as usize];
        }
        return Tile{pipe: 'X', x: -1, y: -1, north: false, south: false, east: false, west: false, dist_from_start: -1, inside: true};
    }
}

fn get_surrounding_tiles(tile: Tile) -> [Tile; 9] {
    let mut tiles: [Tile; 9] = [Tile{pipe: 'X', x: -1, y: -1, north: false, south: false, east: false, west: false, dist_from_start: -1, inside: true}; 9];
    tiles[N] = get_north_tile(tile);
    tiles[S] = get_south_tile(tile);
    tiles[E] = get_east_tile(tile);
    tiles[W] = get_west_tile(tile);

    tiles[NE] = get_north_tile(tiles[E]);
    tiles[NW] = get_north_tile(tiles[W]);
    tiles[SE] = get_south_tile(tiles[E]);
    tiles[SW] = get_south_tile(tiles[W]);

    tiles[C] = tile;

    return tiles;
}

fn set_surrounding_tiles(tiles: [Tile; 9]) {
    unsafe {
        for i in 0..9 {
            if tiles[i].x >= 0 && tiles[i].y >= 0 && tiles[i].x < MAP_SIZE as i32 && tiles[i].y < MAP_SIZE as i32 {
                map[tiles[i].x as usize][tiles[i].y as usize] = tiles[i];
            }
        }
    }
}


fn calc_dist_from_start(x: usize, y: usize, dist: i32) {
    unsafe {
        let mut tile = map[x][y].clone();

        if tile.dist_from_start != -1 && tile.dist_from_start <= dist {
            return;
        }

        tile.dist_from_start = dist;
        map[x][y] = tile;

        println!("Calculating dist for: {:?}", tile);
        if tile.north {
            let north = get_north_tile(tile);
            calc_dist_from_start(north.x as usize, north.y as usize, dist+1);
        }

        if tile.south {
            let south = get_south_tile(tile);
            calc_dist_from_start(south.x as usize, south.y as usize, dist+1);
        }

        if tile.east {
            let east = get_east_tile(tile);
            calc_dist_from_start(east.x as usize, east.y as usize, dist+1);
        }

        if tile.west {
            let west = get_west_tile(tile);
            calc_dist_from_start(west.x as usize, west.y as usize, dist+1);
        }
    }
}

fn get_next_direction(tile: Tile, oe: [bool; 9]) -> usize {
    let outside: usize = get_oe(oe);
    match outside {
        N => { //Heading West: Anything possible except EAST
            if tile.west {
                return W;
            }
            if tile.north {
                return N;
            }
            if tile.south {
                return S;
            }
        },
        S => { //Heading East: Anything possible except WEST
            if tile.east {
                return E;
            }
            if tile.north {
                return N;
            }
            if tile.south {
                return S;
            }
        },
        E => { //Heading North: Anything possible except SOUTH
            if tile.north {
                return N;
            }
            if tile.east {
                return E;
            }
            if tile.west {
                return W;
            }
        },
        W => { //Heading South: Anything possible except NORTH
            if tile.south {
                return S;
            }
            if tile.east {
                return E;
            }
            if tile.west {
                return W;
            }
        }
        _ => {
            println!("Invalid Outside Edge");
        }
    }

    //CRASH HERE
    println!("CRASH - Tile: {:?}", tile);
    println!("CRASH - Outside Edge: {:?}", oe);
    return C;
}

fn get_oe(oe: [bool; 9]) -> usize {
    let mut outside: usize = C;

    if oe[N] {
        outside = N;
    } else if oe[S] {
        outside = S;
    } else if oe[E] {
        outside = E;
    } else if oe[W] {
        outside = W;
    }
    return outside;
}

fn get_ie(oe: [bool; 9]) -> usize {
    let outside = get_oe(oe);
    match outside {
        N => {
            return S;
        },
        S => {
            return N;
        },
        E => {
            return W;
        },
        W => {
            return E;
        },
        _ => {
            return C;
        }
    }
}

fn update_oe(oe: [bool; 9], direction: usize) -> [bool; 9] {
    let mut new = [false; 9];
    let outside: usize = get_oe(oe);

    match outside {
        N => {
            match direction { //Heading West
                N => {
                    //Heading West, turning North = East
                    new[E] = true;
                    new[SE] = true;
                },
                E => {
                    //Invalid Direction
                    println!("Invalid Direction - W to E");
                },
                S => {
                    //Heading West, turning South = West
                    new[W] = true;
                    new[NW] = true;
                },
                W => {
                    // Same Direction
                    new[N] = true;
                },
                _ => {
                    println!("Invalid Direction - W to {}", direction);
                }
            }
        },
        E => { //Heading North
            match direction {
                N => {
                    //Same Direction
                    new[E] = true;
                },
                E => {
                    //Heading North, turning East = South
                    new[S] = true;
                    new[SW] = true;
                },
                S => {
                    //Heading North, turning South = Invalid
                    println!("Invalid Direction - N to S");
                },
                W => {
                    //Heading North, turning West = North
                    new[N] = true;
                    new[NE] = true;
                },
                _ => {
                    println!("Invalid Direction - S to {}", direction);
                }
            }
        },
       S => { //Heading East
            match direction {
                N => {
                    //Heading East, turning North = East
                    new[E] = true;
                    new[SE] = true;
                },
                E => {
                    //Same Direction
                    new[S] = true;
                },
                S => {
                    //Heading East, turning South = West
                    new[W] = true;
                    new[NW] = true;
                },
                W => {
                    //Heading East, turning West = Invalid
                    println!("Invalid Direction - E to W");
                },
                _ => {
                    println!("Invalid Direction - E to {}", direction);
                }
            }
        },
        W => { //Heading South
            match direction {
                N => {
                    //Invalid Direction
                    println!("Invalid Direction - S to N");
                },
                E => {
                    //Heading South, turning East = South
                    new[S] = true;
                    new[SW] = true;
                },
                S => {
                    //Same Direction
                    new[W] = true;
                },
                W => {
                    //Heading South, turning West = North
                    new[N] = true;
                    new[NE] = true;
                },
                _ => {
                    println!("Invalid Direction - S to {}", direction);
                }
            }
        },
        _ => {
            println!("Invalid Outside Edge");
        }

    }
    return new;
}

fn mark_outside(tile: Tile) {
    let mut tiles = get_surrounding_tiles(tile);
    if tiles[C].pipe == '.' {
        tiles[C].pipe = 'O';
        tiles[C].inside = false;
    }
    set_surrounding_tiles(tiles);
}

fn mark_inside(tile: Tile) {
    let mut tiles = get_surrounding_tiles(tile);
    if tiles[C].pipe == '.' {
        tiles[C].pipe = 'I';
        tiles[C].inside = true;
    }
    set_surrounding_tiles(tiles);
}

fn flood_fill(tile: Tile) {
    if tile.pipe == 'O' || tile.pipe == 'I'{
        let mut tiles = get_surrounding_tiles(tile);
        for i in 0..9 {
            if tiles[i].pipe == '.' {
                tiles[i].pipe = tiles[C].pipe;
                tiles[i].inside = tiles[C].inside;
            }
        }
        set_surrounding_tiles(tiles);
    }
}

fn count_pipes(pipe: char) -> i32 {
    unsafe {
    let mut count = 0;
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            if map[x][y].pipe == pipe {
                count += 1;
            }
        }
    }
    return count;
    }
}


fn walk_path(start_x: i32, start_y: i32) {
    let mut tile = get_tile(start_x, start_y);
    let mut oe: [bool; 9] = [false; 9]; //Outside Edge
    for i in 0..9 {
        let t = get_surrounding_tiles(tile)[i];
        if t.pipe == 'O' {
            oe[i] = true;
        }
        if t.pipe == 'X' { //Outside of the map
            oe[i] = true;
        }
    }

    let mut visited: i32 = 0;
    loop {
        println!("Tile: {:?}", tile);
        let tiles = get_surrounding_tiles(tile);
        for i in 0..9 {
            if oe[i]  {
                mark_outside(tiles[i]);
            }
        }
        mark_inside(tiles[get_ie(oe)]);
        visited += 1;
        let direction = get_next_direction(tile, oe);
        if direction == C {
            break;
        }
        oe = update_oe(oe, direction);
        tile = tiles[direction];
        if tile.x == start_x && tile.y == start_y {
            //Been all the way around
            println!("All the way around = {}", visited);
            break;
        }
    }

}

static mut map: [[Tile; MAP_SIZE]; MAP_SIZE] = [[Tile{pipe:'X', x: -1, y: -1, north: false, south: false, east: false, west: false, dist_from_start: -1, inside: false}; MAP_SIZE]; MAP_SIZE];

fn main() {
    //Map is a 2d array of Tiles: 300x300
    unsafe {
        let file = std::fs::read_to_string("input.txt").unwrap();
        let mut x = 0;
        let mut y = 0;
        for line in file.lines() {
            x = 0;
            for c in line.chars() {
                map[x][y] = Tile{pipe: c, x: x as i32, y: y as i32, north: false, south: false, east: false, west: false, dist_from_start: -1, inside: false};
                x += 1;
            }
            y += 1;
        }

        let mut start_x = MAP_SIZE;
        let mut start_y = MAP_SIZE;
        for y in 0..y {
            for x in 0..x {
                let mut tile = map[x][y].clone();
                match tile.pipe {
                    '|' => {
                        if y > 0 {
                            tile.north = true;
                        }
                        if y < (MAP_SIZE-1) {
                            tile.south = true;
                        }
                        //convert to ascii value
                        tile.pipe = '|';
                    },
                    '-' => {
                        if x > 0 {
                            tile.west = true;
                        }
                        if x < (MAP_SIZE-1) {
                            tile.east = true;
                        }
                        //convert to ascii value
                        tile.pipe = '-';
                    },
                    'L' => {
                        if y > 0 {
                            tile.north = true;
                        }
                        if x < (MAP_SIZE-1){
                            tile.east = true;
                        }
                        tile.pipe = '+';
                    },
                    'J' => {
                        if y > 0 {
                            tile.north = true;
                        }
                        if x > 0 {
                            tile.west = true;
                        }
                        tile.pipe = '+';
                    },
                    '7' => {
                        if y < (MAP_SIZE-1) {
                            tile.south = true;
                        }
                        if x > 0 {
                            tile.west = true;
                        }
                        tile.pipe = '+';
                    },
                    'F' => {
                        if y < (MAP_SIZE-1) {
                            tile.south = true;
                        }
                        if x < (MAP_SIZE-1) {
                            tile.east = true;
                        }
                        tile.pipe = '+';
                    },
                    'S' => {
                        start_x = x;
                        start_y = y;
                    },
                    _ => {}
                }
                map[x][y] = tile;
            }
        }


        let mut start_tile = map[start_x][start_y].clone();
        if start_y > 0 {
            let north = map[start_x][start_y-1].clone();
            //if North is a tile and it has a south neighbor
            if north.south {
                start_tile.north = true;
            }
        }
        if start_y < (MAP_SIZE-1) {
            let south = map[start_x][start_y+1].clone();
            //if South is a tile and it has a north neighbor
            if south.north {
                start_tile.south = true;
            }
        }
        if start_x > 0 {
            let west = map[start_x-1][start_y].clone();
            //if West is a tile and it has an east neighbor
            if west.east {
                start_tile.west = true;
            }
        }
        if start_x < (MAP_SIZE-1) {
            let east = map[start_x+1][start_y].clone();
            //if East is a tile and it has a west neighbor
            if east.west {
                start_tile.east = true;
            }
        }

        map[start_x][start_y] = start_tile;
        println!("Start: {:?}", map[start_x][start_y]);

        calc_dist_from_start(start_x, start_y, 0);

        //Print the map
        //
        for y in 0..y {
            for x in 0..x {
                if map[x][y].dist_from_start == -1 {
                    print!(" ");
                    map[x][y].pipe = '.';
                } else {
                    print!("{}", map[x][y].pipe);
                }
            }
            println!();
        }

        //Find the largest distance from start
        let mut max_dist = 0;
        for y in 0..y {
            for x in 0..x {
                if map[x][y].dist_from_start > max_dist {
                    max_dist = map[x][y].dist_from_start;
                }
            }
        }

        println!("Max distance from start: {}", max_dist);

        //Mark the tiles as I = Inside, O = Outside, . = Connected
        let mut start_x = MAP_SIZE;
        let mut start_y = MAP_SIZE;

        for y in 0..y {
            for x in 0..x {
                let tile = map[x][y];
                if tile.dist_from_start == -1 {
                    mark_outside(tile);
                } else {
                    if start_x == MAP_SIZE && start_y == MAP_SIZE  {
                        start_x = x;
                        start_y = y;
                    }
                    break;
                }
                map[x][y] = tile;
            }
        }

        for y in 0..y {
            for x in (0..x).rev() {
                let tile = map[x][y];
                if tile.dist_from_start == -1 {
                    mark_outside(tile);
                } else {
                    break;
                }
                map[x][y] = tile;
            }
        }
        
        for x in 0..x {
            for y in 0..y {
                let tile = map[x][y];
                if tile.dist_from_start == -1 {
                    mark_outside(tile);
                } else {
                    break;
                }
                map[x][y] = tile;
            }
        }

        for x in 0..x {
            for y in (0..y).rev() {
                let tile = map[x][y];
                if tile.dist_from_start == -1 {
                    mark_outside(tile);
                } else {
                    break;
                }
                map[x][y] = tile;
            }
        }

        let start_tile = map[start_x][start_y].clone();
        println!("Outside Start: {:?}", start_tile);

        walk_path(start_x as i32, start_y as i32);

        let mut dots = count_pipes('.');
        while dots > 0 {
            for y in 0..y {
                for x in 0..x {
                    let tile = map[x][y];
                    flood_fill(tile);
                }
            }
            let new_dots = count_pipes('.');
            if new_dots == dots {
                //Print DOTS
                for y in 0..y {
                    for x in 0..x {
                        if map[x][y].pipe == '.' {
                            println!("{}: {:?}", map[x][y].pipe, map[x][y]);
                        }
                    }
                }
                break;
            }
            dots = new_dots;
            println!("Dots: {}", dots);
        }

        //Print the map        
        for y in 0..y {
            for x in 0..x {
                print!("{}", map[x][y].pipe);
            }
            println!();
        }

        let inclusions = count_pipes('I');
        println!("Inclusions: {}", inclusions);
    }
}
