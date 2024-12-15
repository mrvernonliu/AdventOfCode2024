use std::collections::{VecDeque};
use std::fs;
use log::{debug, info};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BlockType {
    FileBlock,
    EmptyBlock
}

#[derive(Debug, Copy, Clone)]
struct Block {
    block_type: BlockType,
    id: i32,
    length: i32
}

impl Block {
    fn new(block_type: BlockType, id: i32, length: i32) -> Self {
        Self {
            block_type,
            id,
            length
        }
    }
}

pub fn part_1() {
    let input = fs::read_to_string("./resources/day9.txt")
        .expect("Failed to load file");
    debug!("Input: {}", input);
    let mut block_list: VecDeque<Block> = VecDeque::new();

    let mut file_index = 0;
    for (index, c) in input.chars().enumerate() {
        if index % 2 == 0 {
            block_list.push_back(Block::new(
                BlockType::FileBlock,
                file_index,
                c.to_digit(10).expect("char was not a number") as i32));
            file_index += 1;
        } else {
            block_list.push_back(Block::new(
                BlockType::EmptyBlock,
                -1,
                c.to_digit(10).expect("char was not a number") as i32));
        }
    }
    debug!("Block list: {:?}", block_list);

    let mut result_list: Vec<i32> = Vec::new();
    let mut current_block =  block_list.pop_front().expect("Failed to deque block");
    let mut end_block: Block = block_list.pop_back().expect("Failed to pop block off the end");
    while !block_list.is_empty() {
        if current_block.length == 0 {
            current_block = match block_list.pop_front() {
                None => break,
                Some(block) => block
            };
        }
        debug!("Current block: {:?}", current_block);
        if current_block.block_type == BlockType::FileBlock {
            for _ in 0..current_block.length {
                result_list.push(current_block.id);
            }
            current_block.length = 0;
            continue
        } else {
            if end_block.block_type == BlockType::EmptyBlock {
                end_block = match block_list.pop_back() {
                    None => break,
                    Some(block) => block
                };
                continue
            }
            if current_block.length < end_block.length {
                for _ in 0..current_block.length {
                    result_list.push(end_block.id);
                }
                end_block.length -= current_block.length;
                current_block.length = 0;
            } else {
                for _ in 0..end_block.length {
                    result_list.push(end_block.id);
                }
                current_block.length -= end_block.length;
                end_block = match block_list.pop_back() {
                    None => break,
                    Some(block) => block
                };
            }
        }
    }
    for _ in 0..end_block.length {
        result_list.push(end_block.id);
    }
    debug!("Result: {:?}", result_list);

    let mut hash: u64 = 0;
    for (index, id) in result_list.iter().enumerate() {
        hash += index as u64 * *id as u64;
    }
    info!("Part 1: {}", hash);
}

/*
    This is... terrible, don't look at it.
 */
pub fn part_2() {
    let input = fs::read_to_string("./resources/day9.txt")
        .expect("Failed to load file");
    debug!("Input: {}", input);
    let mut block_list: Vec<Block> = Vec::new();
    let mut file_index = 0;
    for (index, c) in input.chars().enumerate() {
        if index % 2 == 0 {
            block_list.push(Block::new(
                BlockType::FileBlock,
                file_index,
                c.to_digit(10).expect("char was not a number") as i32));
            file_index += 1;
        } else {
            block_list.push(Block::new(
                BlockType::EmptyBlock,
                file_index, // Add index so that we know if we can insert a file here
                c.to_digit(10).expect("char was not a number") as i32));
        }
    }
    debug!("Original Block list: {:?}", block_list);

    let mut end_index = block_list.len();
    while end_index > 0 {
        end_index -= 1;
        debug!("{}", get_block_string(&block_list));
        if block_list.get(end_index).is_some() {
            if block_list.get(end_index).unwrap().block_type == BlockType::EmptyBlock {
                continue
            }
        } else {
            break;
        }
        debug!("End index: {}", end_index);

        for i in 0..block_list.len(){
            let block = &block_list[i];
            let curr_end_file = &block_list[end_index];
            if block.block_type == BlockType::FileBlock {
                continue
            }
            debug!("Comparing: {:?} to {:?}", block, curr_end_file);
            if block.length >= curr_end_file.length && i < end_index {
                debug!("Moving {}", curr_end_file.id);
                if block.length == curr_end_file.length {
                    block_list.swap(i, end_index);
                } else {
                    let new_empty_block = Block{
                        block_type: BlockType::EmptyBlock,
                        id: block_list[i].id,
                        length: curr_end_file.length,
                    };
                    block_list[i].length = block_list[i].length - curr_end_file.length;
                    block_list.insert(i, new_empty_block);
                    block_list.swap(i, end_index+1);
                }
                break;
            }
        }
    }

    let mut hash: u64 = 0;
    let mut index = 0;
    for block in &block_list {
        if block.block_type == BlockType::EmptyBlock {
            index += block.length;
        } else {
            for i in index..(block.length+index) {
                hash += block.id as u64 * i as u64;
            }
            index += block.length;
        }
    }

    info!("Part 2: {}", hash);
}

fn get_block_string(block_list: & Vec<Block>) -> String {
    let mut index = 0;
    let mut print_string = String::new();
    for block in block_list {
        if block.block_type == BlockType::EmptyBlock {
            for _ in index..(block.length + index) {
                print_string.push_str(".");
            }
            index += block.length;
        } else {
            for _ in index..(block.length + index) {
                print_string.push_str(&format!("{}", block.id));
            }
            index += block.length;
        }
    }
    print_string
}

// fn repeat_char(key: String, length: i32) -> String {
//     let id_string = &format!("{}", key);
//     std::iter::repeat(id_string.clone()).take(length as usize).collect()
// }
