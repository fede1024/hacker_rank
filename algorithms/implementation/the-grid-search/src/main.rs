use std::io;
use std::io::prelude::*;

struct Map {
    height: usize,
    width: usize,
    data: Vec<i32>,
}

impl Map {
    fn new(height: usize, width: usize) -> Map {
        Map { width: width, height: height, data: vec![0; width * height] }
    }
}

fn read_map<T: Iterator<Item=String>>(lines: &mut T) -> Map {
    let line = lines.next().unwrap();
    let nums = line.split(" ").collect::<Vec<_>>();
    let height = nums[0].parse::<usize>().unwrap();
    let width = nums[1].parse::<usize>().unwrap();
    let mut map = Map::new(height, width);
    for y in 0..height {
        let line = lines.next().unwrap();
        for x in 0..width {
            map.data[y * width + x] = line[x..x+1].parse::<i32>().unwrap();
        }
    }
    map
}

fn read_maps() -> Vec<(Map, Map)> {
    let mut maps = Vec::new();
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines().map(|x| x.unwrap());
    let pairs_num = lines_iter.next().unwrap().parse::<i32>().unwrap();
    for _ in 0..pairs_num {
        let outer_map = read_map(&mut lines_iter);
        let inner_map = read_map(&mut lines_iter);
        maps.push((outer_map, inner_map));
    }
    maps
}

fn search_map(outer: &Map, inner: &Map) -> bool {
    for oy in 0..(outer.height - inner.height + 1) {
        for ox in 0..(outer.width - inner.width + 1) {
            let mut found = true;
            'search: for iy in 0..inner.height {
                for ix in 0..inner.width {
                    let outer_data = outer.data[(oy + iy) * outer.width + ox + ix];
                    let inner_data = inner.data[iy * inner.width + ix];
                    if outer_data != inner_data {
                        found = false;
                        break 'search;
                    }
                }
            }
            if found {
                return true
            }
        }
    }
    false
}

fn main() {
    for (outer, inner) in read_maps() {
        if search_map(&outer, &inner) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
