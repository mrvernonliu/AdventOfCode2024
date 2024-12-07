use std::collections::{HashMap, HashSet};
use std::fs;
use log::{debug, info};

pub fn part_1_and_2() {
    let (dependency_map, updates) = load_input();

    let mut valid_updates: Vec<Vec<i32>> = Vec::new();
    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        let mut completed_pages: HashSet<i32> = HashSet::new();
        let mut is_valid_update = true;
        for page in &update {
            let dependencies = match dependency_map.get(&page) {
                Some(dependencies) => dependencies,
                None => {
                    completed_pages.insert(*page);
                    continue
                }
            };
            for dependency in dependencies {
                if update.contains(dependency) && !completed_pages.contains(dependency) {
                    debug!("{}'s dependency {} not found in completed pages: {:?}", page, dependency, completed_pages);
                    is_valid_update = false;
                    break;
                }
            }
            completed_pages.insert(*page);
        }
        if is_valid_update {
            debug!("pushing valid update");
            valid_updates.push(update);
        } else {
            debug!("pushing invalid update");
            invalid_updates.push(update)
        }
    }
    debug!("Valid Updates: {:?}", valid_updates);
    print_middle_sum(&valid_updates);

    part_2(&dependency_map, &invalid_updates);
}

// Could be better but it works for now...
pub fn part_2(dependency_map: &HashMap<i32, HashSet<i32>>, invalid_updates: &Vec<Vec<i32>>) {
    // info!("Dependency map: {:?}", dependency_map);
    let mut corrected_updates: Vec<Vec<i32>> = vec![Vec::new(); invalid_updates.len()];

    for update in invalid_updates {
        corrected_updates.push(process_update(dependency_map, &update));
    }
    debug!("Corrected Updates: {:?}", corrected_updates);
    print_middle_sum(&corrected_updates)
}

pub fn process_update(dependency_map: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut required_dependencies: HashMap<i32, HashSet<i32>> = HashMap::new();
    for page in update {
        let mut filtered_dependencies: HashSet<i32> = HashSet::new();
        let all_dependencies = match dependency_map.get(page) {
            Some(set) => set,
            None => continue
        };
        for dependency in all_dependencies {
            if update.contains(dependency) {
                filtered_dependencies.insert(*dependency);
            }
        }
        required_dependencies.insert(*page, filtered_dependencies);
    }

    // Do this in two steps for easier debugging - we can combine this with the previous
    // loop if we want though
    let mut all_insertion_orders: Vec<Vec<i32>> = Vec::new();
    let mut placed_dependencies: HashSet<i32> = HashSet::new();
    for page in update {
        let insertion_order = prepend_dependencies_recursively(*page, &required_dependencies, &mut placed_dependencies);
        debug!("insertion order given page {} - {:?}", page, insertion_order);
        all_insertion_orders.push(insertion_order)
    }

    let mut flattened_insertions: Vec<i32> = Vec::new();
    for array in all_insertion_orders {
        let mut filtered: Vec<i32> = array.into_iter().filter(|x| !flattened_insertions.contains(x)).collect();
        flattened_insertions.append(&mut filtered);
    }
    flattened_insertions
}

fn prepend_dependencies_recursively(page: i32, required_dependencies: &HashMap<i32, HashSet<i32>>, placed_dependencies: &mut HashSet<i32>) -> Vec<i32> {
    let mut insertion_order: Vec<i32> = Vec::new();
    let page_dependencies = match required_dependencies.get(&page) {
        None => {
            placed_dependencies.insert(page);
            insertion_order.push(page);
            return insertion_order;
        }
        Some(set) => set
    };
    for dependency in page_dependencies {
        if placed_dependencies.contains(dependency) {
            continue
        }
        insertion_order.append(&mut prepend_dependencies_recursively(*dependency, required_dependencies,  placed_dependencies ));
    }
    placed_dependencies.insert(page);
    insertion_order.push(page);
    insertion_order
}

fn load_input() -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut dependency_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let input = match fs::read_to_string("./resources/day5.txt") {
        Ok(input) => input,
        Err(_) => panic!("Failed to read file")
    };
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            let x: i32 = parts[0].parse().unwrap();
            let y: i32 = parts[1].parse().unwrap();
            let dependency_set = dependency_map.entry(y).or_insert_with(HashSet::new);
            dependency_set.insert(x);
        }
        if line.contains(',') {
            let pages: Vec<i32> = line.split(',')
                .map(|s| {
                    s.trim().parse().unwrap()
                })
                .collect();
            updates.push(pages);
        }
    };
    // info!("Updates: {:?}", updates);
    (dependency_map, updates)
}

fn print_middle_sum(updates: &Vec<Vec<i32>>) {
    let middle_sum: i32 = updates.iter().filter_map(|valid_update| {
        return valid_update.get(valid_update.len()/2);
    })
        .sum();
    info!("Middle Sum: {}", middle_sum);
}

// [[42, 86, 74, 92, 83, 47, 78, 12, 26, 54, 46, 85, 14, 11, 43, 62, 69, 57, 76, 66, 48]]
// [[42, 86, 62, 12, 46, 85, 54, 78, 26, 83, 43, 92, 14, 11, 47, 74, 69, 57, 76, 66, 48]]