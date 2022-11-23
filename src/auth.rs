use home_config::HomeConfig;

fn read_config() {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig");
    let data = config.read_to_string().unwrap();
    println!("Config: {}", data);
}


fn set_config() {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig");
    config.save("123456789").unwrap();
}


pub fn handle_auth() {
    set_config();
    read_config();

}