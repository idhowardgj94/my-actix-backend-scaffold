use yaml_rust::YamlLoader;
use std::fs;
use std::rc::Rc;
pub struct Env {
    pub server_url: String,
    pub db_type: String,
    pub user: String,
    pub password: String,
    pub db: String,
    pub db_url: String,
}

pub fn get_from(str: &str) -> Env {
    let env =
        YamlLoader::load_from_str(&fs::read_to_string(str).unwrap());
    if let Ok(v) = env {
        let doc = v.get(0).unwrap().to_owned();
        Env {
            server_url: doc["server"]["url"].clone().into_string().unwrap(),
            db_type: doc["db"]["type"].clone().into_string().unwrap(),
            db_url: doc["db"]["url"].clone().into_string().unwrap(),
            db: doc["db"]["db"].clone().into_string().unwrap(),
            user: doc["db"]["user"].clone().into_string().unwrap(),
            password: doc["db"]["password"].clone().into_string().unwrap(),
        }
    } else {
        panic!("parse error");
    }

}
