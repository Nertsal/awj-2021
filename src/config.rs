use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub teeth_locations: Vec<Vec2<f32>>,
}

impl geng::LoadAsset for Config {
    const DEFAULT_EXT: Option<&'static str> = Some("json");

    fn load(geng: &Geng, path: &str) -> geng::AssetFuture<Self> {
        let future = String::load(geng, path);
        Box::pin(async move { Ok(serde_json::from_str(&future.await?).unwrap()) })
    }
}
