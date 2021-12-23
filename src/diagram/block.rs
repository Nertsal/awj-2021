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
        emit_positions: Vec<Position>,
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
                                let pos = delta + self.position.map(|x| x as isize);
                                if pos.x < 0 || pos.y < 0 {
                                    None
                                } else {
                                    Some((pos.map(|x| x as usize), direction))
                                }
                            })
                            .collect(),
                    })
            }
            BlockType::Source {
                signal_color,
                emit_positions,
                ..
            } => Some(BlockAction::EmitSignal {
                color: *signal_color,
                positions: emit_positions
                    .iter()
                    .map(|&position| (position, Directions::all()))
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
                *queued_signal = Some((signal_color, connections.and(signal_directions)));
            }
            BlockType::Source { .. } => (),
        }
    }
}
