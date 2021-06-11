use std::fs;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::io;

#[derive(Deserialize, Serialize)]
struct Gamepaths {
    skyrimse: Option<Gamepath>,     //1
    skyrim: Option<Gamepath>,       //2
    oblivion: Option<Gamepath>,     //3
    fallout4: Option<Gamepath>,     //4
    falloutnv: Option<Gamepath>,    //5
    fallout3: Option<Gamepath>,     //6
}

#[derive(Deserialize, Serialize)]
pub struct Gamepath {
    pub data: String,
    pub plugins: String,
    pub mods: String,
}

fn create_conf_file(conf: &str) -> io::Result<()> {
    let dir = format!("{}{}", env::var("HOME").unwrap(), "/config/rmm2/");
    fs::create_dir_all(dir.clone())?;
    fs::write(conf, "[Gamepaths]")
}

fn write_conf_file(config: Gamepaths) -> io::Result<()> {
    let conf = format!("{}{}",env::var("HOME").unwrap(), "/.config/rmm2/config");
    let content = format!("[Gamepaths]\n\n{}", toml::to_string(&config).unwrap());
    fs::write(conf, content)
}

fn create_game_conf(mode: usize) -> Gamepath {
    let mut d = String::new();
    println!("Enter the path to your game's data directory (absolute path)");
    io::stdin().read_line(&mut d).unwrap();

    Gamepath {
        data: fix_data_path(&d),
        plugins: get_plugin_path(&d, mode),
        mods: get_mod_path(&d),
    }
}

fn fix_data_path(path: &str) -> String {
    let mut buff = String::from("/");
    for i in path.split('/') {
        if i == "Data" || i == "data" {
            buff.push_str("Data/");
            break;
        }
        else {
            buff.push_str(i);
            buff.push('/');
        }
        
    }
    buff
}

fn get_mod_path(path: &str) -> String {
    let mut buff = String::from("/");
    for i in path.split('/') {
        if i == "Data" {
            buff.push_str("Mods/");
            break;
        }
        else {
            buff.push_str(i);
            buff.push('/');
        }
        
    }
    fs::create_dir_all(&buff).unwrap();
    buff
}

fn get_plugin_path(path: &str, mode: usize) -> String {
    let mut buff = String::from("/");
    let paths = vec![
        "",
        "",
        "",
        "",
        "",
        "",
    ];

    for i in path.split('/') {
        if i == "steamapps" {
            return format!("{}{}", buff, paths[mode]);
        }
        else {
            buff.push_str(i);
            buff.push('/');
        }
    }

    println!("Could not find plugins file. Please enter the path manually (absolute path)");

    let mut buf2 = String::new();
    io::stdin().read_line(&mut buf2).unwrap();
    buff
}

fn read_toml(conf: &str, mode: usize) -> Gamepath {
    let mut paths: Gamepaths = toml::from_str(conf).unwrap();
    match mode {
        1 => match paths.skyrimse {
            Some(x) => x,
            None => {
                paths.skyrimse = Some(create_game_conf(1));
                write_conf_file(paths).unwrap();
                read_toml(conf, mode)
            }
        }
        2 => match paths.skyrim {
            Some(x) => x,
            None => {
                paths.skyrim = Some(create_game_conf(1));
                write_conf_file(paths).unwrap();
                read_toml(conf, mode)
            }
        }
        3 => match paths.oblivion {
            Some(x) => x,
            None => {
                paths.oblivion = Some(create_game_conf(1));
                write_conf_file(paths).unwrap();
                read_toml(conf, mode)
            }
        }
        4 => match paths.fallout4 {
            Some(x) => x,
            None => {
                paths.fallout4 = Some(create_game_conf(1));
                write_conf_file(paths).unwrap();
                read_toml(conf, mode)
            }
        }
        5 => match paths.falloutnv {
            Some(x) => x,
            None => {
                paths.falloutnv = Some(create_game_conf(1));
                write_conf_file(paths).unwrap();
                read_toml(conf, mode)
            }
        }
        6 => match paths.fallout3 {
            Some(x) => x,
            None => {
                paths.fallout3 = Some(create_game_conf(1));
                write_conf_file(paths).unwrap();
                read_toml(conf, mode)
            }
        }
        _default => panic!("Invalid mode!"),
    }
}

pub fn read_config(mode: usize) -> Gamepath {
    let conf = format!("{}{}",env::var("HOME").unwrap(), "/.config/rmm2/config");

    match fs::read_to_string(&conf) {
        Ok(x) => read_toml(&x, mode), 
        _default => {
            create_conf_file(&conf).unwrap(); 
            read_config(mode)
        }
    }
}



