mod block;
mod directions;

pub use block::*;
pub use directions::*;

use super::*;

pub type Position = Vec2<usize>;
pub type RectPos = AABB<usize>;

type BlockId = u32;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CellState {
    Null,
    Empty,
    Occupied(BlockId),
}

#[derive(Serialize, Deserialize)]
pub struct Diagram {
    cell_map: Vec<Vec<CellState>>,
    blocks: HashMap<BlockId, Block>,
    next_id: BlockId,
}

impl Diagram {
    pub fn new(size: Vec2<usize>) -> Self {
        Self {
            cell_map: vec![vec![CellState::Empty; size.x]; size.y],
            blocks: HashMap::new(),
            next_id: 0,
        }
    }

    fn next_id(&mut self) -> BlockId {
        let id = self.next_id;
        self.next_id += 1;
        id
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

    fn get_cell_mut_at(&mut self, position: Position) -> Option<&mut CellState> {
        self.cell_map
            .get_mut(position.y)
            .and_then(|row| row.get_mut(position.x))
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

    pub fn insert_block_at(&mut self, position: Position, mut block: Block) -> bool {
        block.shift(position);
        let positions = block.positions();
        if !positions
            .iter()
            .all(|position| matches!(self.get_cell_at(*position), CellState::Empty))
        {
            return false;
        }

        let id = self.next_id();
        for position in positions {
            *self.get_cell_mut_at(position).unwrap() = CellState::Occupied(id);
        }
        self.blocks.insert(id, block);

        true
    }

    pub fn clear_at(&mut self, position: Position) {
        if let Some(block_id) = self.get_block_id_at(position) {
            for position in self.blocks.remove(&block_id).unwrap().positions() {
                *self.get_cell_mut_at(position).unwrap() = CellState::Empty;
            }
        }
    }

    pub fn load_from_file(
        file_path: impl AsRef<std::path::Path>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_reader(std::fs::File::open(file_path)?)?)
    }

    pub fn save_to_file(
        &self,
        file_path: impl AsRef<std::path::Path>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        serde_json::to_writer(std::fs::File::create(file_path)?, self)?;
        Ok(())
    }
}
