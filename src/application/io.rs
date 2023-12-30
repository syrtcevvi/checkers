/*
Copyright 2023 Сырцев Вадим Игоревич

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

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
pub fn restore_vcs_from_file() -> Result<Vcs> {
    let file = OpenOptions::new().read(true).open(VCS_FILE_PATH)?;
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
