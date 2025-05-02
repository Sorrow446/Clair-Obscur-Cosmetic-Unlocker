mod utils;
mod consts;

use std::fs::File;
use std::path::PathBuf;
use std::error::Error;
use std::{env, fs, io};
use std::io::BufWriter;
use uesave::{MapEntry, PropertyInner, PropertyValue, Save};

fn open_save(path: &PathBuf) -> Result<Save, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let save = Save::read(&mut f)?;
    Ok(save)
}

fn write_save(save: &Save, out_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut f = File::create(out_path)?;
    let mut bw = BufWriter::new(&mut f);
    save.write(&mut bw)?;
    Ok(())
}

fn inject_items(s: &mut Save) -> Result<(), Box<dyn Error>> {
    if let PropertyInner::Map(items) = &mut s.root.properties["InventoryItems"].inner {
        for item_string in consts::ITEM_STRINGS {
            let e = MapEntry{
                key: PropertyValue::Name(item_string.to_string()),
                value: PropertyValue::Int(1),
            };

            if !items.contains(&e) {
                println!("-> {}", item_string);
                items.push(e);
            }
        }

    } else {
        return Err("inventory items key is missing".into());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let in_path_str = env::args().nth(1).ok_or("input save path wasn't provided")?;
    let in_path = PathBuf::from(&in_path_str);
    let temp_out_path = utils::get_temp_path();

    {
        let mut s = open_save(&in_path)?;
        inject_items(&mut s)?;
        write_save(&s, &temp_out_path)?;
    }

    fs::remove_file(&in_path)?;
    fs::copy(&temp_out_path, &in_path)?;
    fs::remove_file(&temp_out_path)?;

    println!("OK. Press enter to exit...");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(())
}
