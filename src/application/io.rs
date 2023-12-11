use crate::application::structs::Vcs;
use serde::Serialize;
use std::{
    fs::OpenOptions,
    io::{BufReader, Result, Write},
    path::Path,
};

/// Путь к файлу, в котором будет сохранено состояние СКВ
const VCS_FILE_PATH: &str = "vcs.data";

/// Восстанавливает из файла состояние СКВ
pub fn restore_vcs_from_file(path: impl AsRef<Path>) -> Result<Vcs> {
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);

    Ok(bincode::deserialize_from(reader).unwrap())
}

/// Записывает сериализованный объект СКВ в файл
pub fn persist_vcs_in_file(vcs: &Vcs) -> Result<()> {
    persist_in_file(VCS_FILE_PATH, vcs)
}

/// Записывает сериализуемый объект в выходной файл
fn persist_in_file(path: impl AsRef<Path>, value: &impl Serialize) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;

    file.write(&bincode::serialize(value).unwrap())?;

    file.flush()?;

    Ok(())
}
