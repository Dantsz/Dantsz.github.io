use std::{env, fs, path::Path, ffi::OsStr};
fn main() -> std::io::Result<()>
{
    for entry in fs::read_dir(Path::new("resources/articles"))? {
        println!("{:?}",entry);
    }
    let articles = fs::read_dir(Path::new("resources/articles"))?
    .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_owned())
    .filter(|name| (Path::new(name).extension().unwrap_or(OsStr::new("")).eq("md")))
    .collect::<Vec<String>>()
    .join(",");

    //read current article list and check for modifications
    match fs::read("resources/articles/articles.csv")
    {
        Ok(list) =>
        {
            
            match list.as_slice().eq(articles.as_bytes())
            {
                true => Ok(()),
                false => fs::write("resources/articles/articles.csv",articles),
            }
        },
        Err(_) => fs::write("resources/articles/articles.csv",articles),
    }
}