use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub top_teeth_locations: Vec<Vec2<f32>>,
    pub bottom_teeth_locations: Vec<Vec2<f32>>,
    pub face_radius: f32,
    pub stick_size: Vec2<f32>,
    pub tooth_size: Vec2<f32>,
    pub tooth_edge_size: f32,
    pub stick_move_speed: f32,
    pub stick_hit_radius: f32,
    pub crumb_speed: f32,
    pub crumb_size: Vec2<f32>,
    pub starting_crubs: usize,
}

impl Config {
    pub fn stick_height(&self) -> f32 {
        self.tooth_size.y - self.face_radius
    }
}

impl geng::LoadAsset for Config {
    const DEFAULT_EXT: Option<&'static str> = Some("json");

    fn load(geng: &Geng, path: &str) -> geng::AssetFuture<Self> {
        let future = String::load(geng, path);
        Box::pin(async move { Ok(serde_json::from_str(&future.await?).unwrap()) })
    }
}
