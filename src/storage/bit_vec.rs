pub enum BitState {
    On,
    Off
}

pub enum BitVecError {
    OutOfBounds {
        length: usize
    }
}

pub struct BitVec {
    blocks: Vec<usize>
}

impl BitVec {
    /// Creates a new empty bit vector.
    pub const fn new() -> BitVec {
        Self {
            blocks: Vec::new()
        }
    }

    /// Creates a new bit vector with preallocated bits (rounded up to the block size).
    pub fn with_bits(amount: usize) -> BitVec {
        Self {
            blocks: vec![0; compute_blocks_for_bits(amount)]
        }
    }

    /// Creates a new bit vector with preallocated blocks of bits.
    pub fn with_blocks(amount: usize) -> BitVec {
        Self {
            blocks: vec![0; amount]
        }
    }

    /// Returns length of bit vector in bits (how many bits you can use without triggering additional
    /// allocations).
    pub fn len(&self) -> usize {
        self.blocks.len() * usize::BITS as usize
    }

    // Returns state of bit at the given index.
    pub fn get(&self, index: usize) -> Result<BitState, BitVecError> {
        if index >= self.len() {
            return Err(BitVecError::OutOfBounds{
                length: self.len()
            });
        }

        let block = self.blocks[compute_block(index)];

        match block >> compute_shift(index) & 1 {
            1 => Ok(BitState::On),
            _ => Ok(BitState::Off)
        }
    }

    /// Sets bit at index in the given state.
    /// 
    /// It will allocate more blocks if index you trying to reach to
    /// is bigger then currently allocated amount of bits.
    pub fn set(&mut self, index: usize, state: BitState) {
        if index >= self.len() {
            self.blocks.resize(compute_blocks_for_bits(index), 0);
        }

        match state {
            BitState::On => {
                self.blocks[compute_block(index)] |= 1 << compute_shift(index);
            },
            BitState::Off => {
                self.blocks[compute_block(index)] &= !(1 << compute_shift(index));
            }
        }
    }

    /// Flips bit at given index in the opposite state.
    /// 
    /// It will allocate more blocks if index you trying to reach to
    /// is bigger then currently allocated amount of bits.
    pub fn flip(&mut self, index: usize) {
        if index >= self.len() {
            self.blocks.resize(compute_blocks_for_bits(index), 0);
        }

        self.blocks[compute_block(index)] ^= 1 << compute_shift(index);
    }
}

// Returns shift size for the given bit index.
const fn compute_shift(bit_index: usize) -> usize {
    bit_index % usize::BITS as usize
}

// Returns block index for the given bit index.
const fn compute_block(bit_index: usize) -> usize {
    bit_index / usize::BITS as usize
}

// Returns how many blocks needed for the given amount of bits to fit them all.
const fn compute_blocks_for_bits(amount: usize) -> usize {
    (amount + usize::BITS as usize) / usize::BITS as usize
}