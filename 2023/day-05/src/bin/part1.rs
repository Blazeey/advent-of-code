use std::cmp::min;
use std::collections::BTreeMap;
use std::ops::Bound::*;
use std::u64::MAX;

#[derive(Clone, Debug)]
struct Range {
    src: u64,
    dst: u64,
    len: u64,
}

fn to_range(content: &str) -> Range {
    let split: Vec<_> = content
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    Range {
        src: split[1],
        dst: split[0],
        len: split[2],
    }
}

fn create_map(ranges: Vec<Range>) -> BTreeMap<u64, Range> {
    let mut map: BTreeMap<u64, Range> = BTreeMap::new();
    // map.insert(0, dummy_range);
    for range in ranges.iter().enumerate() {
        map.insert(range.1.src, range.1.clone());
    }
    map
}

fn get_mapped_value(map: &BTreeMap<u64, Range>, key: u64) -> u64 {
    if map.contains_key(&key) {
        return map.get(&key).unwrap().dst;
    }
    let previous_value = map.range((Unbounded, Excluded(key))).next_back();
    if previous_value.is_none() {
        key
    } else {
        let range = previous_value.unwrap().1;
        if range.src + range.len >= key {
            range.dst + key - range.src
        } else {
            key
        }
    }
}

fn split_lines(content: &str) -> u64 {
    let lines: Vec<_> = content.lines().collect();

    let seeds: Vec<_> = lines[0]
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
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
    for seed in seeds.iter() {
        let soil = get_mapped_value(&map_seed_soil, *seed);
        let fertilizer = get_mapped_value(&map_soil_fertilizer, soil);
        let water = get_mapped_value(&map_fertilizer_water, fertilizer);
        let light = get_mapped_value(&map_water_light, water);
        let temperature = get_mapped_value(&map_light_temperature, light);
        let humidity = get_mapped_value(&map_temperture_humidity, temperature);
        let location = get_mapped_value(&map_humidity_location, humidity);
        println!("{location}");
        min_location = min(min_location, location)
    }
    min_location
}

fn main() {
    let file_content = include_str!("../../input1.txt");
    let min_location = split_lines(file_content);
    println!("{min_location}");
}
