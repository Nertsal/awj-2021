use super::*;

#[derive(Serialize, Deserialize)]
struct ConfigSer {
    pub teeth_locations: Vec<Vec2<f32>>,
    pub face_size: f32,
    pub stick_size: Vec2<f32>,
    pub tooth_size: Vec2<f32>,
    pub stick_move_speed: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(from = "ConfigSer")]
pub struct Config {
    pub teeth_locations: Vec<Vec2<f32>>,
    pub face_size: f32,
    pub stick_size: Vec2<f32>,
    pub tooth_size: Vec2<f32>,
    pub stick_height: f32,
    pub stick_move_speed: f32,
}

impl From<ConfigSer> for Config {
    fn from(config: ConfigSer) -> Self {
        Self {
            teeth_locations: config.teeth_locations,
            face_size: config.face_size,
            stick_size: config.stick_size,
            tooth_size: config.tooth_size,
            stick_move_speed: config.stick_move_speed,
            stick_height: config.tooth_size.y - config.face_size,
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
