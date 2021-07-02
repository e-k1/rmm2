use std::fs;
use std::io;

pub mod utils;
use crate::paths::Path;
use crate::ui::selection_menu;
use crate::files::read_datadir;


fn unpack(src: &Path, dest: &Path) -> io::Result<()> {

    let file = fs::File::open(src.as_str())?;
    let mut archive = zip::ZipArchive::new(&file)?;

    //testi
    let full_size: f32 = file.metadata()?.len() as f32;
    let mut file_size: f32 = 0.0;

    for i in 0..archive.len() {
        let mut path = dest.as_str().clone();
        let mut file = archive.by_index(i)?;        
            match file.enclosed_name() {
                Some(pth) => pth.to_owned(),
                None => continue,
            };

            path.push_str(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&path)?;
            }
            else {
                let mut outfile = fs::File::create(&path)?;
                io::copy(&mut file,&mut outfile)?;
                file_size += outfile.metadata()?.len() as f32;
            }
            println!("{}%", (file_size/(full_size*2.0))*100.0);
        };
    println!("100.0%");
    Ok(())
}

fn move_files_all(src: &Path, dest: &Path) -> io::Result<()> {
    let contents: Vec<String> = read_datadir(src).unwrap();
    for i in 0..contents.len() {

        let src_p = src.clone().push(&contents[i]);
        let dest_p = dest.clone().push(&contents[i]);

        if src_p.is_dir() {
            fs::create_dir_all(&dest_p.as_str())?;
            move_files_all(&src_p, &dest_p)?;
        }
        else {
            fs::rename(src_p.as_str(), dest_p.as_str())?;
        }
    }
    Ok(())
}

fn install_fomod_files(plugin: &utils::FomodPlugin, src: &Path, dest: &Path) -> io::Result<()> {
    for i in 0..plugin.files.len() {
        let src_p = src.clone().push_p(plugin.files[i].source.clone());
        let dest_p = dest.clone().push_p(plugin.files[i].destination.clone());

        if plugin.files[i].ftype == "file" {
            if !dest_p.is_dir() {
                fs::create_dir_all(dest_p.previous().as_str())?;
            }
            fs::rename(src_p.as_str(), dest_p.as_str())?;
        }
        else if plugin.files[i].ftype == "folder" {
            move_files_all(&src_p, &dest_p)?;
        }
    }
    Ok(())
}


/*
fn selection() -> Vec<usize> {
    let mut selected: Vec<usize> = Vec::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    for i in input.split_whitespace() {
        selected.push(i.parse::<usize>().unwrap() - 1);
    }
    selected
}

fn print_plugins(group: &utils::FomodGroup) {
    for i in 0..group.plugins.len() {
        println!("{}) {}", i + 1, group.plugins[i].name);
    }
}
*/


fn install_fomod(src: &Path, dest: &Path) -> io::Result<()> {
    let src_p = src.clone().next();

    let groups = utils::read_install_instructions(src);
        for i in 0..groups.len() {

            let sclt = selection_menu(&groups[i]).unwrap();

            for k in 0..sclt.len() {
                install_fomod_files(&groups[i].plugins[sclt[k]], &src_p, &dest)?;
            }

       }

    fs::remove_dir_all(src_p.as_str())?;
    Ok(())

}

fn install_non_fomod(src: &Path, dest: &Path) -> io::Result<()> {
    let src = src.clone().next();
    utils::dir::cap_dir(&src)?;
    move_files_all(&src, &dest)?;
    fs::remove_dir_all(src.as_str())?;
    Ok(())
}

pub fn install_mod(src: Path, dest: Path) -> io::Result<()> {
    let mut src_p = src.clone();

    if !src.is_dir() {
        src_p = src.previous().push("temp/");
        match unpack(&src, &src_p) {
            Err(_e) => {
                println!("Error extracting. Please extract manually and use the installer.");
                return Err(_e);
            }
            Ok(_x) => {
            fs::remove_file(src.as_str()).unwrap();
            println!("Installing");
            }
        }
    }

    if utils::dir::check_if_fomod(&src_p) {
        install_fomod(&src_p, &dest)?;
    }
    else {
        install_non_fomod(&src_p, &dest)?;
    }       
    Ok(())
}


