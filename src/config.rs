use super::*;

#[derive(Serialize, Deserialize)]
struct ConfigSer {
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(from = "ConfigSer")]
pub struct Config {
    pub top_teeth_locations: Vec<Vec2<f32>>,
    pub bottom_teeth_locations: Vec<Vec2<f32>>,
    pub face_radius: f32,
    pub stick_size: Vec2<f32>,
    pub tooth_size: Vec2<f32>,
    pub tooth_edge_size: f32,
    pub stick_height: f32,
    pub stick_move_speed: f32,
    pub stick_hit_radius: f32,
    pub crumb_speed: f32,
    pub crumb_size: Vec2<f32>,
}

impl From<ConfigSer> for Config {
    fn from(config: ConfigSer) -> Self {
        Self {
            top_teeth_locations: config.top_teeth_locations,
            bottom_teeth_locations: config.bottom_teeth_locations,
            face_radius: config.face_radius,
            stick_size: config.stick_size,
            tooth_size: config.tooth_size,
            tooth_edge_size: config.tooth_edge_size,
            stick_move_speed: config.stick_move_speed,
            stick_hit_radius: config.stick_hit_radius,
            crumb_size: config.crumb_size,
            crumb_speed: config.crumb_speed,
            stick_height: config.tooth_size.y - config.face_radius,
        }
    }
}

impl geng::LoadAsset for Config {
    const DEFAULT_EXT: Option<&'static str> = Some("json");

    fn load(geng: &Geng, path: &str) -> geng::AssetFuture<Self> {
        let future = String::load(geng, path);
        Box::pin(async move { Ok(serde_json::from_str(&future.await?).unwrap()) })
    }
}
