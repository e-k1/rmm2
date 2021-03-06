use std::fs;
use std::io;

pub mod utils;
use crate::paths::Path;
use crate::ui::selection_menu;

use crate::ui::utils::{keyin};

//For unpacking compressed archives. Experimental, fails most of the time.
fn unpack(src: &Path, dest: &Path) -> io::Result<()> {

    let file = fs::File::open(src.as_str())?;
    let mut archive = zip::ZipArchive::new(&file)?;

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
            println!("{}%", (file_size/(full_size*2.5))*100.0);
        };
    println!("100.0%");
    Ok(())
}

fn install_fomod(src: &Path, dest: &Path) -> io::Result<()> {
    let srcr = utils::dir::mod_root(src);
    utils::dir::cap_dir_all(&srcr)?;

    let mut c_flags: Vec<utils::ConditionFlag> = Vec::new();
    let modconfig = utils::read_install_instructions(&srcr, &dest);

    modconfig.install_req_files()?;

    for i in modconfig.installsteps.iter() {
        if i.check(&c_flags) {
            for j in i.groups.iter() {
                let sclt = selection_menu(j).unwrap();
                j.install_plugins(&sclt)?;
                j.get_flags(&sclt, &mut c_flags);
                println!("Press enter to continue"); keyin();
            }
        }
    }

    modconfig.install_conditionals(c_flags)?;

    fs::remove_dir_all(src.as_str())?;
    Ok(())

}

fn install_non_fomod(src: &Path, dest: &Path) -> io::Result<()> {
    utils::dir::cap_dir(&src)?;
    let srcr = utils::dir::mod_root(src);
    utils::dir::move_files_all(&srcr, &dest)?;
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
            Ok(_x) => { fs::remove_file(src.as_str()).unwrap(); }
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


