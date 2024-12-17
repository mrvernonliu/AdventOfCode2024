use std::collections::{HashMap, VecDeque};
use std::{fs, thread};
use std::ops::{Add, Sub};
use std::thread::JoinHandle;
use log::{debug, info};
use num_bigint::BigInt;
use regex::Regex;
use num_traits::{Zero};

const COST_OF_A: i32 = 3;
const COST_OF_B: i32 = 1;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct MachineDefinition {
    button_a: Coordinate,
    button_b: Coordinate,
    prize: Coordinate
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate{x, y}
    }
    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl Sub for &Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate::new(
            self.x - rhs.x,
            self.y - rhs.y
        )
    }
}

impl Add for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate::new(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

impl MachineDefinition {
    fn new(button_a: Coordinate, button_b: Coordinate, prize: Coordinate) -> MachineDefinition {
        MachineDefinition{button_a, button_b, prize}
    }
}

// This is just the coin change problem
pub fn part_1() {
    let definitions = load_input();
    debug!("Definitions: {:?}", definitions);
    let mut total_cost = 0;
    let mut handles: Vec<JoinHandle<i32>> = Vec::new();

    // Added threading for fun
    for definition in definitions {
        handles.push(thread::spawn(move || {
            let mut memoizer: HashMap<Coordinate, Option<i32>> = HashMap::new(); // Memoize <position, minimum cost>
            dfs_with_memoization(&definition.prize, &definition, &mut memoizer, &mut Coordinate::new(0, 0))
                .unwrap_or_else(|| 0)
        }));
    };

    for handle in handles {
        total_cost += handle.join().unwrap();
    }
    info!("Part 1: {}", total_cost);
}

fn dfs_with_memoization(remaining: &Coordinate, machine_definition: &MachineDefinition, memoizer: &mut HashMap<Coordinate, Option<i32>>, presses: &Coordinate) -> Option<i32> {
    debug!("Remaining: {:?}, presses: {:?}", remaining, presses);
    if remaining.is_zero() {
        debug!("Found a solution: {:?}", presses);
        return Some(0);
    }
    if presses.x == 100 || presses.y == 100 {
        debug!("Too many presses");
        return None
    }
    if remaining.x < 0 || remaining.y < 0 {
        debug!("Invalid position");
        return None
    }
    if memoizer.contains_key(remaining) {
        debug!("Found in memoizer: {:?}", remaining);
        return *memoizer.get(remaining).unwrap();
    }

    let a_cost: Option<i32> = match dfs_with_memoization(&(remaining - &machine_definition.button_a), machine_definition, memoizer, &(presses + &Coordinate::new(1, 0))) {
        Some(cost) => Some(cost + COST_OF_A),
        None => None
    };
    let b_cost: Option<i32> = match dfs_with_memoization(&(remaining - &machine_definition.button_b), machine_definition, memoizer, &(presses + &Coordinate::new(0, 1))) {
        Some(cost) => Some(cost + COST_OF_B),
        None => None
    };
    let lowest_cost = match (a_cost, b_cost) {
        (Some(a_cost), Some(b_cost)) => Some(a_cost.min(b_cost)),
        (Some(a_cost), None) => Some(a_cost),
        (None, Some(b_cost)) => Some(b_cost),
        _ => None
    };
    debug!("Memoizing: {:?}: {:?}", remaining, lowest_cost);
    memoizer.insert(remaining.clone(), lowest_cost);
    lowest_cost
}

/*
    Part 2 is unsolvable as I'll OOM before I get anywhere close.
    This is basically a math 9 problem.

    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400

    94A + 22B = 8400
    34A + 67B = 5400

    Now we multiply to eliminate a variable
    34(94A + 22B) = 34(8400) ==> X2(94A + 22B) = X2(8400)
    94(34A + 67B) = 94(5400) ==> X1(34A + 67B) = X1(5400)

    3196A + 748B = 285600
    3196A + 6298B = 507600

    This simplifies to:

    285600 - 748B = 507600 - 6298B
    5550B = 222000
    B = 40

    So back to the definition:
    Button A: X+94, Y+34 ==> Let's call X+94 ax, at Y+35 ay
    Button B: X+22, Y+67 ==> Similarly bx and by
    Prize: X=8400, Y=5400 ==> px, py

    So the equation for B is:
    B = (ax*py - ay*px) / (ax*by - ay*bx)
    B = (94*5400 - 34*8400) / (94*67 - 34*22) = 40 Correct!

    Now we can fill in this in to solve for A or just do in backwards.
    A = (bx*py - by*px) / (bx*ay - by*ax)

    Let's further simplify this down for implementation.
    AorB = (x1*py - y1*px) / (x1*y2 - y1*x2) and we can just flip the order
 */

pub fn part_2() {
    let definitions = load_input();
    let mut total_cost: BigInt = BigInt::from(0);
    let offset = b"10000000000000";

    for definition in definitions {
        let px: BigInt = BigInt::parse_bytes(offset, 10).unwrap() + definition.prize.x;
        let py: BigInt = BigInt::parse_bytes(offset, 10).unwrap()  + definition.prize.y;
        let b = apply_formula(
            definition.button_a.x,
            definition.button_a.y,
            definition.button_b.x,
            definition.button_b.y,
            &px,
            &py
        );
        let a = apply_formula(
            definition.button_b.x,
            definition.button_b.y,
            definition.button_a.x,
            definition.button_a.y,
            &px,
            &py
        );
        if a == None || b == None {
            continue
        }
        total_cost += (a.unwrap() * COST_OF_A) + (b.unwrap() * COST_OF_B);
    }
    info!("Part 2: {}", total_cost);
}

fn apply_formula(x1: i32, y1: i32, x2: i32, y2: i32, px: &BigInt, py: &BigInt) -> Option<BigInt> {
    if !((x1*py - y1*px) % (x1*y2 - y1*x2)).is_zero() {
     return None
    }
    Some((x1*py - y1*px) / (x1*y2 - y1*x2))
}

fn load_input() -> Vec<MachineDefinition> {
    let input_string = fs::read_to_string("./resources/day13.txt").expect("failed to load file");
    let mut definitions: Vec<MachineDefinition> = Vec::new();
    let mut lines: VecDeque<String> = input_string.lines().map(String::from).collect();

    let re = Regex::new(r"(\d+),.*\D(\d+)").unwrap();
    while !lines.is_empty() {
        let button_a_str = lines.pop_front().unwrap();
        let button_b_str = lines.pop_front().unwrap();
        let prize_str = lines.pop_front().unwrap();
        lines.pop_front(); // skip empty line
        debug!("Button A: {}, Button B: {}, Prize: {}", button_a_str, button_b_str, prize_str);

        let button_a_regex = re.captures(&*button_a_str).expect("Failed to parse A");
        let button_b_regex = re.captures(&*button_b_str).expect("Failed to parse B");
        let prize_regex = re.captures(&*prize_str).expect("Failed to parse B");

        let button_a = Coordinate::new(
            button_a_regex.get(1).expect("Failed to read X from A").as_str().parse().unwrap(),
            button_a_regex.get(2).expect("Failed to read Y from A").as_str().parse().unwrap()
        );
        let button_b = Coordinate::new(
            button_b_regex.get(1).expect("Failed to read X from B").as_str().parse().unwrap(),
            button_b_regex.get(2).expect("Failed to read Y from B").as_str().parse().unwrap()
        );
        let prize = Coordinate::new(
            prize_regex.get(1).expect("Failed to read X from Prize").as_str().parse().unwrap(),
            prize_regex.get(2).expect("Failed to read Y from Prize").as_str().parse().unwrap()
        );
        definitions.push(
            MachineDefinition::new(button_a, button_b, prize)
        );

    }
    definitions
}