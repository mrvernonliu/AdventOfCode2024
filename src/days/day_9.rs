use std::collections::VecDeque;
use std::fs;
use log::{debug, info};

#[derive(Debug, PartialEq, Eq)]
enum BlockType {
    FileBlock,
    EmptyBlock
}

#[derive(Debug)]
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

fn repeat_char(key: i32, length: i32) -> String {
    let id_string = &format!("{}", key);
    std::iter::repeat(id_string.clone()).take(length as usize).collect()
}
