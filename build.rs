use std::{ffi::OsStr, fs, path::Path};
fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(Path::new("resources/posts"))? {
        println!("{:?}", entry);
    }
    let articles = fs::read_dir(Path::new("resources/posts"))?
        .map(|entry| entry.unwrap().file_name().to_str().unwrap().to_owned())
        .filter(|name| {
            Path::new(name)
                .extension()
                .unwrap_or(OsStr::new(""))
                .eq("md")
        })
        .collect::<Vec<String>>()
        .join(",");

    //read current article list and check for modifications
    match fs::read("resources/posts/posts.csv") {
        Ok(list) => match list.as_slice().eq(articles.as_bytes()) {
            true => Ok(()),
            false => fs::write("resources/posts/posts.csv", articles),
        },
        Err(_) => fs::write("resources/posts/posts.csv", articles),
    }
}
