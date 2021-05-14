use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub aws_access_key_id: String,
    pub aws_url: String,
    pub aws_secret_access_key: String,
    pub aws_default_region: String,
    pub aws_bucket: String,
    pub download_path: String,
    pub aws_objects_prefix: Option<String>
}

lazy_static! {
    pub static ref CONFIG: Config = envy::from_env::<Config>().unwrap();
}
