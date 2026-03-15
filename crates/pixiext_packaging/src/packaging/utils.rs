use std::fs;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;
use zip::CompressionMethod;
use zip::write::FileOptions;

pub fn zip_dir<T>(
    dir: &Path,
    writer: T,
    method: CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    fn recurse<T>(
        path: &Path,
        base: &Path,
        zip: &mut zip::ZipWriter<T>,
        options: FileOptions,
        buffer: &mut Vec<u8>,
    ) -> zip::result::ZipResult<()>
    where
        T: Write + Seek,
    {
        if path != base {
            let name = path.strip_prefix(base).unwrap();
            let dir_name = format!("{}/", path_to_zip_name(name));
            println!("Adding directory: {:?}", dir_name);
            zip.add_directory(dir_name, options)?;
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                let name = entry_path.strip_prefix(base).unwrap();
                let file_name = path_to_zip_name(name);
                println!("Adding file {:?} as {:?} ...", entry_path, file_name);
                zip.start_file(file_name, options)?;
                let mut f = File::open(&entry_path)?;
                f.read_to_end(buffer)?;
                zip.write_all(&*buffer)?;
                buffer.clear();
            } else if entry_path.is_dir() {
                recurse(&entry_path, base, zip, options, buffer)?;
            }
        }
        Ok(())
    }

    recurse(dir, dir, &mut zip, options, &mut buffer)?;
    zip.finish()?;
    Ok(())
}

fn path_to_zip_name(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
