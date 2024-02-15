const FILE_NAME: &str = ".todo.txt";

pub fn get_file_path() -> String {
    // match env::var("HOME") {
    //     Ok(home) => format!("{}/{}", &home, FILE_NAME),
    //     Err(_) => format!("./{}", FILE_NAME),
    // }
    format!("./{}", FILE_NAME)
}
