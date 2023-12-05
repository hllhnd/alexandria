// Copyright (c) 2023 Reperak
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::env::current_dir;
use std::fs::create_dir_all;
use std::fs::rename;
use std::io::stdin;
use std::path::PathBuf;

use claxon::FlacReader;
use walkdir::WalkDir;

fn main()
{
    let mut mappings: HashMap<PathBuf, PathBuf> = HashMap::new();

    for file in WalkDir::new(current_dir().unwrap())
        .into_iter()
        .filter_map(|res| res.ok())
        .filter(|dir| dir.path().extension().unwrap_or_default() == "flac")
        .map(|dir| dir.path().to_owned())
    {
        let reader = FlacReader::open(&file).unwrap();

        let mut new_path = current_dir().unwrap();
        new_path.push(reader.get_tag("ARTIST").next().unwrap_or_default().replace('/', "_"));
        new_path.push(reader.get_tag("ALBUM").next().unwrap_or_default().replace('/', "_"));
        new_path.push(reader.get_tag("TITLE").next().unwrap_or_default().replace('/', "_"));

        new_path.set_extension("flac");

        mappings.insert(file, new_path);
    }

    for (old_path, new_path) in mappings.iter().filter(|(old_path, new_path)| old_path != new_path) {
        println!(
            "{} -> {}",
            old_path.strip_prefix(current_dir().unwrap()).unwrap().display(),
            new_path.strip_prefix(current_dir().unwrap()).unwrap().display()
        );
    }

    let mut input = String::new();
    println!("Confirm? (y/N)");
    stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "y" | "Y" =>
            for (old_path, new_path) in mappings.iter() {
                create_dir_all(new_path.parent().unwrap()).unwrap();
                rename(old_path, new_path).unwrap();
            },
        _ => println!("Aborted"),
    }
}
