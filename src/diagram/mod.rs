mod block;
mod directions;

pub use block::*;
pub use directions::*;

use super::*;

pub type Position = Vec2<usize>;
pub type RectPos = AABB<usize>;

type BlockId = u32;

#[derive(Clone, Copy, Debug)]
pub enum CellState {
    Null,
    Empty,
    Occupied(BlockId),
}

pub struct Diagram {
    cell_map: Vec<Vec<CellState>>,
    blocks: HashMap<BlockId, Block>,
}

impl Diagram {
    pub fn new(size: Vec2<usize>) -> Self {
        Self {
            cell_map: vec![vec![CellState::Empty; size.x]; size.y],
            blocks: HashMap::new(),
        }
    }

    pub fn tick(&mut self) {
        for action in self
            .blocks
            .values_mut()
            .filter_map(|block| block.tick())
            .collect::<Vec<_>>()
        {
            match action {
                BlockAction::EmitSignal { color, positions } => {
                    for (position, directions) in positions {
                        if let Some(block) = self.get_block_mut_at(position) {
                            block.receive_signal(color, directions);
                        }
                    }
                }
            }
        }
    }

    pub fn map_width(&self) -> usize {
        self.cell_map
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or_default()
    }

    pub fn map_height(&self) -> usize {
        self.cell_map.len()
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.values()
    }

    /// Returns the cell state at a given position.
    pub fn get_cell_at(&self, position: Position) -> CellState {
        self.cell_map
            .get(position.y)
            .and_then(|row| row.get(position.x).copied())
            .unwrap_or(CellState::Null)
    }

    fn get_block_id_at(&self, position: Position) -> Option<BlockId> {
        match self.get_cell_at(position) {
            CellState::Occupied(block_id) => Some(block_id),
            _ => None,
        }
    }

    /// Returns a reference to a block that occupies the position
    /// if there is a block there.
    pub fn get_block_at(&self, position: Position) -> Option<&Block> {
        self.get_block_id_at(position)
            .map(|block_id| self.blocks.get(&block_id).expect(&format!("Cell map appears to be in an illegal state: a block with the id {} exists in the map at position {}, but it is unknown", block_id, position)))
    }

    /// Returns a mutable reference to a block that occupies the position
    /// if there is a block there.
    fn get_block_mut_at(&mut self, position: Position) -> Option<&mut Block> {
        self.get_block_id_at(position)
            .map(|block_id| self.blocks.get_mut(&block_id).expect(&format!("Cell map appears to be in an illegal state: a block with the id {} exists in the map at position {}, but it is unknown", block_id, position)))
    }
}
