use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub position: Position,
    pub block_type: BlockType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockType {
    Wire {
        connections: Directions,
        queued_signal: Option<(SignalColor, Directions)>,
    },
    Source {
        signal_color: SignalColor,
        emit_directions: Directions,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SignalColor {
    Red,
    Green,
    Blue,
}

impl SignalColor {
    pub fn color_f32(&self) -> Color<f32> {
        match self {
            SignalColor::Red => Color::RED,
            SignalColor::Green => Color::GREEN,
            SignalColor::Blue => Color::BLUE,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockAction {
    EmitSignal {
        color: SignalColor,
        positions: Vec<(Position, Directions)>,
    },
}

impl Block {
    pub fn tick(&mut self) -> Option<BlockAction> {
        match &mut self.block_type {
            BlockType::Wire { queued_signal, .. } => {
                queued_signal
                    .take()
                    .map(|(color, directions)| BlockAction::EmitSignal {
                        color,
                        positions: directions
                            .deltas()
                            .filter_map(|(delta, direction)| {
                                shift_position(self.position, delta).map(|pos| (pos, direction))
                            })
                            .collect(),
                    })
            }
            BlockType::Source {
                signal_color,
                emit_directions,
                ..
            } => Some(BlockAction::EmitSignal {
                color: *signal_color,
                positions: emit_directions
                    .deltas()
                    .filter_map(|(delta, direction)| {
                        shift_position(self.position, delta).map(|pos| (pos, direction))
                    })
                    .collect(),
            }),
        }
    }

    pub fn receive_signal(&mut self, signal_color: SignalColor, signal_directions: Directions) {
        match &mut self.block_type {
            BlockType::Wire {
                connections,
                queued_signal,
                ..
            } => {
                *queued_signal = Some((signal_color, *connections - signal_directions.opposite()));
            }
            BlockType::Source { .. } => (),
        }
    }
}
