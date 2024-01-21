use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use typescript_type_def::{DefinitionFileOptions, write_definition_file_from_type_infos, TypeDef};

#[path = "src/state/message.rs"]
mod message;

#[path = "src/state/user.rs"]
mod user;

fn get_root_path() -> Option<PathBuf> {
    let out_dir = env::var("OUT_DIR").ok()?;
    Some(Path::new(&out_dir).parent()?
        .parent()?
        .parent()?
        .parent()?
        .parent()?.to_path_buf())
}

fn main() {

    let options = DefinitionFileOptions::default();


    let out_dir = env::var("OUT_DIR").expect("Out dir should exist");

    // Yes. I know.
    let dest_path = get_root_path().expect("Root path should exist")
        .join("client").join("src").join("types");
    let _ = std::fs::create_dir_all(&dest_path);

    let type_infos = vec![
        &message::MessageDTO::INFO,
        &user::UserDTO::INFO,
    ];

    let mut f = File::create(&dest_path.clone().join("ppapi.ts")).unwrap();
    write_definition_file_from_type_infos(&mut f, options, &type_infos).unwrap();
}