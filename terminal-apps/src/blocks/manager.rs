use super::block::Block;
use std::collections::VecDeque;

pub struct BlockHistory {
    blocks: VecDeque<Block>,
    max_size: usize,
    current_index: usize,
}

impl BlockHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            blocks: VecDeque::new(),
            max_size,
            current_index: 0,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        if self.blocks.len() >= self.max_size {
            self.blocks.pop_front();
        } else {
            self.current_index = self.blocks.len();
        }
        self.blocks.push_back(block);
    }

    pub fn get_blocks(&self) -> Vec<&Block> {
        self.blocks.iter().collect()
    }

    pub fn get_block(&self, id: &str) -> Option<&Block> {
        self.blocks.iter().find(|b| b.id == id)
    }

    pub fn get_block_mut(&mut self, id: &str) -> Option<&mut Block> {
        self.blocks.iter_mut().find(|b| b.id == id)
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }

    pub fn navigate_up(&mut self) -> Option<&Block> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.blocks.get(self.current_index)
        } else {
            None
        }
    }

    pub fn navigate_down(&mut self) -> Option<&Block> {
        if self.current_index < self.blocks.len() - 1 {
            self.current_index += 1;
            self.blocks.get(self.current_index)
        } else {
            None
        }
    }

    pub fn search_by_command(&self, query: &str) -> Vec<&Block> {
        self.blocks
            .iter()
            .filter(|b| b.command.contains(query))
            .collect()
    }

    pub fn get_bookmarked(&self) -> Vec<&Block> {
        self.blocks.iter().filter(|b| b.is_bookmarked()).collect()
    }

    pub fn clear(&mut self) {
        self.blocks.clear();
        self.current_index = 0;
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}

pub struct BlockManager {
    history: BlockHistory,
}

impl BlockManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            history: BlockHistory::new(max_history),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.history.add_block(block);
    }

    pub fn get_blocks(&self) -> Vec<&Block> {
        self.history.get_blocks()
    }

    pub fn get_block(&self, id: &str) -> Option<&Block> {
        self.history.get_block(id)
    }

    pub fn get_block_mut(&mut self, id: &str) -> Option<&mut Block> {
        self.history.get_block_mut(id)
    }

    pub fn search(&self, query: &str) -> Vec<&Block> {
        self.history.search_by_command(query)
    }

    pub fn get_bookmarked(&self) -> Vec<&Block> {
        self.history.get_bookmarked()
    }

    pub fn toggle_bookmark(&mut self, id: &str) -> Result<(), String> {
        self.history
            .get_block_mut(id)
            .ok_or_else(|| "Block not found".to_string())
            .map(|block| block.toggle_bookmark())
    }

    pub fn history(&self) -> &BlockHistory {
        &self.history
    }

    pub fn history_mut(&mut self) -> &mut BlockHistory {
        &mut self.history
    }
}
