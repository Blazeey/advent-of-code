use std::cmp::min;
use std::collections::BTreeMap;
use std::i64::MAX;
use std::ops::Bound::*;

#[derive(Clone, Debug)]
struct Range {
    src: i64,
    dst: i64,
    len: i64,
}

fn to_range(content: &str) -> Range {
    let split: Vec<_> = content
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    Range {
        src: split[1],
        dst: split[0],
        len: split[2],
    }
}

fn create_map(ranges: Vec<Range>) -> BTreeMap<i64, Range> {
    let mut map: BTreeMap<i64, Range> = BTreeMap::new();
    // map.insert(0, dummy_range);
    for range in ranges.iter().enumerate() {
        map.insert(range.1.src, range.1.clone());
    }
    map
}

fn get_mapped_ranges(map: &BTreeMap<i64, Range>, start: i64, end: i64) -> Vec<(i64, i64)> {
    println!("start {:?}, end {:?}", start, end);
    let left_range = map.range((Unbounded, Excluded(start))).next_back();
    let mut range = map.range((Included(start), Included(end)));
    let right_range = map.range((Excluded(start), Unbounded)).next();
    let mut prev_end: i64 = start;
    let mut new_ranges: Vec<(i64, i64)> = vec![];
    println!("{:?}, {:?}", left_range, right_range);
    if left_range.is_some() {
        let (_, r) = left_range.unwrap();
        println!("Previous {:?}", r);
        let mut s = start;
        let mut e: i64 = 0;
        if start >= r.src && start < r.src + r.len {
            s = r.dst + start - r.src;
            prev_end = r.src + r.len;
            if end < r.src + r.len {
                e = r.dst + end - r.src;
            } else {
                e = r.dst + r.len - 1;
            }
            new_ranges.push((s, e));
        }
    }
    println!("VEC {:?}", new_ranges);
    // range.next();
    for (_, r) in range {
        println!("{:?}", r);
        if prev_end < r.src - 1 {
            new_ranges.push((prev_end, r.src - 1));
        }
        let mut e = r.dst + r.len - 1;
        if end < r.src + r.len {
            e = r.dst + end - r.src;
        }
        new_ranges.push((r.dst, e));
        prev_end = r.src + r.len;
        println!("Range {:?}, end {:?}", r, prev_end);
    }
    if prev_end <= end {
        new_ranges.push((prev_end, end));
        println!("LAST PUSH {:?}", new_ranges.last());
    }

    new_ranges
}

fn split_lines(content: &str) -> i64 {
    let lines: Vec<_> = content.lines().collect();

    let seeds: Vec<(i64, i64)> = lines[0]
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| (s[0], s[1]))
        .collect();
    let mut index_seed_soil = 0;
    let mut index_soil_fertilizer = 0;
    let mut index_fertilizer_water = 0;
    let mut index_water_light = 0;
    let mut index_light_temperature = 0;
    let mut index_temperture_humidity = 0;
    let mut index_humidity_location = 0;
    for line in lines.iter().enumerate() {
        match *line.1 {
            "seed-to-soil map:" => index_seed_soil = line.0,
            "soil-to-fertilizer map:" => index_soil_fertilizer = line.0,
            "fertilizer-to-water map:" => index_fertilizer_water = line.0,
            "water-to-light map:" => index_water_light = line.0,
            "light-to-temperature map:" => index_light_temperature = line.0,
            "temperature-to-humidity map:" => index_temperture_humidity = line.0,
            "humidity-to-location map:" => index_humidity_location = line.0,
            _ => (),
        }
    }

    let lines_seed_soil: Vec<Range> = lines[index_seed_soil + 1..index_soil_fertilizer - 1]
        .iter()
        .copied()
        .map(to_range)
        .collect();
    let lines_soil_fertilizer: Vec<Range> = lines
        [index_soil_fertilizer + 1..index_fertilizer_water - 1]
        .iter()
        .copied()
        .map(to_range)
        .collect();
    let lines_fertilizer_water: Vec<Range> = lines
        [index_fertilizer_water + 1..index_water_light - 1]
        .iter()
        .copied()
        .map(to_range)
        .collect();
    let lines_water_light: Vec<Range> = lines[index_water_light + 1..index_light_temperature - 1]
        .iter()
        .copied()
        .map(to_range)
        .collect();
    let lines_light_temperature: Vec<Range> = lines
        [index_light_temperature + 1..index_temperture_humidity - 1]
        .iter()
        .copied()
        .map(to_range)
        .collect();
    let lines_temperture_humidity: Vec<Range> = lines
        [index_temperture_humidity + 1..index_humidity_location - 1]
        .iter()
        .copied()
        .map(to_range)
        .collect();
    let lines_humidity_location: Vec<Range> = lines[index_humidity_location + 1..]
        .iter()
        .copied()
        .map(to_range)
        .collect();

    let map_seed_soil = create_map(lines_seed_soil);
    let map_soil_fertilizer = create_map(lines_soil_fertilizer);
    let map_fertilizer_water = create_map(lines_fertilizer_water);
    let map_water_light = create_map(lines_water_light);
    let map_light_temperature = create_map(lines_light_temperature);
    let map_temperture_humidity = create_map(lines_temperture_humidity);
    let map_humidity_location = create_map(lines_humidity_location);

    let mut min_location = MAX;
    let mut soil_ranges: Vec<(i64, i64)> = vec![];
    let mut fertilizer_ranges: Vec<(i64, i64)> = vec![];
    let mut water_ranges: Vec<(i64, i64)> = vec![];
    let mut light_ranges: Vec<(i64, i64)> = vec![];
    let mut temperature_ranges: Vec<(i64, i64)> = vec![];
    let mut humidity_ranges: Vec<(i64, i64)> = vec![];
    let mut location_ranges: Vec<(i64, i64)> = vec![];

    for seed in seeds.iter() {
        soil_ranges.extend(get_mapped_ranges(
            &map_seed_soil,
            seed.0,
            seed.0 + seed.1 - 1,
        ));
    }
    for r in soil_ranges {
        fertilizer_ranges.extend(get_mapped_ranges(&map_soil_fertilizer, r.0, r.1));
    }
    for r in fertilizer_ranges {
        water_ranges.extend(get_mapped_ranges(&map_fertilizer_water, r.0, r.1));
    }
    for r in water_ranges {
        light_ranges.extend(get_mapped_ranges(&map_water_light, r.0, r.1));
    }
    for r in light_ranges {
        temperature_ranges.extend(get_mapped_ranges(&map_light_temperature, r.0, r.1));
    }
    for r in temperature_ranges {
        humidity_ranges.extend(get_mapped_ranges(&map_temperture_humidity, r.0, r.1));
    }
    for r in humidity_ranges {
        location_ranges.extend(get_mapped_ranges(&map_humidity_location, r.0, r.1));
    }
    // println!("{:?}", soil_ranges);
    for r in location_ranges {
        min_location = min(min_location, r.0)
    }
    min_location
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    let min_location = split_lines(file_content);
    println!("{min_location}");
}
