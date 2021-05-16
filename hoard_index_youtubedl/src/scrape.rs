use {
    crate::InfoJson,
    std::{
        ffi::OsString,
        fs,
        io,
        os::unix::ffi::{
            OsStrExt,
            OsStringExt,
        },
        path::{
            Path,
            PathBuf,
        },
    },
};

/// Scrape the .info.json files in the given directory,
/// returning an iterator of [`InfoJson`] values.
///
/// Each entry also includes the path of the .info.json file,
/// but without the .info.json extension.
/// The appropriate video and thumbnail extensions
/// may be added by the caller if desired.
///
/// [`InfoJson`]: struct.InfoJson.html
pub fn scrape_directory<P>(path: P)
    -> io::Result<impl Iterator<Item=io::Result<(PathBuf, InfoJson)>>>
    where P: AsRef<Path>
{
    Ok(

        // List all files in the directory.
        fs::read_dir(path)?

        // Get the path of each entry.
        // Propagate any I/O errors.
        .map(|entry_result| {
            entry_result.map(|entry| entry.path())
        })

        // Keep only .info.json entries.
        // Also keep any I/O errors.
        .filter(|path_result| {
            match path_result {
                Ok(path) => has_extension(path),
                Err(_) => true,
            }
        })

        // Parse each .info.json file and return it.
        // Also return the path without extension.
        .map(|path_result| {
            path_result.and_then(|path| {
                let info_json = InfoJson::from_file(&path)?;
                let path = pop_extension(path);
                Ok((path, info_json))
            })
        })

    )
}

fn has_extension<P>(path: P) -> bool
    where P: AsRef<Path>
{
    path.as_ref()
        .as_os_str()
        .as_bytes()
        .ends_with(b".info.json")
}

fn pop_extension(pathbuf: PathBuf) -> PathBuf
{
    let mut vec = pathbuf.into_os_string().into_vec();
    vec.resize(vec.len() - ".info.json".len(), 0);
    PathBuf::from(OsString::from_vec(vec))
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_scrape_directory()
    {
        let iter = scrape_directory("testdata").unwrap();

        let mut info_jsons = iter.collect::<Result<Vec<_>, _>>().unwrap();
        info_jsons.sort_by(|(_, a), (_, b)| Ord::cmp(&a.title, &b.title));

        assert_eq!(info_jsons.len(), 2);

        let path_0 = concat!("testdata/Dominoes - HARDCORE Mode - ",
                             "Smarter Every Day 182-9hPIobthvHg");
        let path_1 = concat!("testdata/Fractals are typically not ",
                             "self-similar-gB9n2gHsHN4");
        assert_eq!(info_jsons[0].0, PathBuf::from(path_0));
        assert_eq!(info_jsons[1].0, PathBuf::from(path_1));

        let title_0 = "Dominoes - HARDCORE Mode - Smarter Every Day 182";
        let title_1 = "Fractals are typically not self-similar";
        assert_eq!(info_jsons[0].1.title, title_0);
        assert_eq!(info_jsons[1].1.title, title_1);
    }
}
