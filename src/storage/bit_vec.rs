pub enum BitState {
    On,
    Off
}

pub struct BitVec {
    blocks: Vec<usize>
}

impl BitVec {
    /// Creates a new empty bit vector.
    pub fn new() -> BitVec {
        Self {
            blocks: Vec::new()
        }
    }

    /// Creates a new bit vector with preallocated blocks.
    pub fn with_blocks(blocks: usize) -> BitVec {
        Self {
            blocks: vec![0; blocks]
        }
    }

    /// Returns length of bit vector in bits (how many bits you can use without triggering additional
    /// allocations).
    pub fn len(&self) -> usize {
        self.blocks.len() * usize::BITS as usize
    }

    /// Sets bit at index in given state.
    /// 
    /// It will allocate more blocks if index you trying to reach to
    /// is bigger then currently allocated amount of bits.
    pub fn set(&mut self, index: usize, state: BitState) {
        if index >= self.len() {
            let blocks_length = (index + usize::BITS as usize) / usize::BITS as usize;

            self.blocks.resize(blocks_length, 0);
        }

        let shift = index % usize::BITS as usize;
        let block_index = index / usize::BITS as usize;

        match state {
            BitState::On => {
                self.blocks[block_index] |= 1 << shift;
            },
            BitState::Off => {
                self.blocks[block_index] &= !(1 << shift);
            }
        }
    }
}