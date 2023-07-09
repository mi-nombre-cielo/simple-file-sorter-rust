use walkdir::WalkDir;
use std::collections::HashMap;
use std::env;
use std::fs;

mod split_point_slash;

fn main() {
    let get_current_dir = env::current_dir().unwrap();

    println!("current dir: {}", get_current_dir.display()); 

    let mut expansion_key: String = String::new();
    let mut file_path: String = String::new();
    let mut new_file_path: String = String::new();
    let mut key_folder_hm = HashMap::new();
    let mut file_path_for_split = String::new();
    let mut file_path_for_split_slash = String::new();

    if fs::create_dir("audio").is_ok() { // creating folder
        println!("creating folder...");
    } else if fs::create_dir("audio").is_err() {
        println!("error creating folder");
    }
    if fs::create_dir("video").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("video").is_err() {             
        println!("error creating folder");
    }
    if fs::create_dir("data").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("data").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("image").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("image").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("archive").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("archive").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("text").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("text").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("3d").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("3d").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("presentation").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("presentation").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("spreadsheet").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("spreadsheet").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("font").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("font").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("gif").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("gif").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("exe").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("exe").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("bat").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("bat").is_err() {
        println!("error creating folder");
    } 
    if fs::create_dir("apk").is_ok() {
        println!("creating folder...");
    } else if fs::create_dir("apk").is_err() {
        println!("error creating folder");
    } 
    
    key_folder_hm.insert("mp3","audio"); // specifying key folders
    key_folder_hm.insert("wav","audio");
    key_folder_hm.insert("ogg","audio");
    key_folder_hm.insert("flac","audio");
    key_folder_hm.insert("aif","audio");
    key_folder_hm.insert("mid","audio");
    key_folder_hm.insert("midi","audio");
    key_folder_hm.insert("mpa","audio");
    key_folder_hm.insert("wma","audio");
    key_folder_hm.insert("wpl","audio");
    key_folder_hm.insert("cda","audio");
    
    key_folder_hm.insert("mp4","video"); 
    key_folder_hm.insert("mov","video"); 
    key_folder_hm.insert("avi","video"); 
    key_folder_hm.insert("mkv","video"); 
    key_folder_hm.insert("wmv","video"); 
    key_folder_hm.insert("3g2","video"); 
    key_folder_hm.insert("mpg","video"); 
    key_folder_hm.insert("mpeg","video"); 
    key_folder_hm.insert("m4v","video"); 
    key_folder_hm.insert("h264","video"); 
    key_folder_hm.insert("flv","video"); 
    key_folder_hm.insert("rm","video"); 
    key_folder_hm.insert("swf","video"); 
    key_folder_hm.insert("vob","video"); 

    key_folder_hm.insert("sql","data"); 
    key_folder_hm.insert("sqlite","data"); 
    key_folder_hm.insert("sqlite3","data"); 
    key_folder_hm.insert("csv","data"); 
    key_folder_hm.insert("dat","data"); 
    key_folder_hm.insert("db","data"); 
    key_folder_hm.insert("log","data"); 
    key_folder_hm.insert("sav","data"); 
    key_folder_hm.insert("tar","data"); 
    key_folder_hm.insert("xml","data"); 
    
    key_folder_hm.insert("jpg","image"); 
    key_folder_hm.insert("png","image"); 
    key_folder_hm.insert("bmp","image"); 
    key_folder_hm.insert("ai","image"); 
    key_folder_hm.insert("psd","image"); 
    key_folder_hm.insert("psdx","image"); 
    key_folder_hm.insert("ico","image"); 
    key_folder_hm.insert("jpeg","image"); 
    key_folder_hm.insert("ps","image"); 
    key_folder_hm.insert("svg","image"); 
    key_folder_hm.insert("tif","image"); 
    key_folder_hm.insert("tiff","image"); 

    key_folder_hm.insert("zip","archive"); 
    key_folder_hm.insert("rar","archive"); 
    key_folder_hm.insert("7z","archive"); 
    key_folder_hm.insert("z","archive"); 
    key_folder_hm.insert("gz","archive"); 
    key_folder_hm.insert("rpm","archive"); 
    key_folder_hm.insert("arj","archive"); 
    key_folder_hm.insert("pkg","archive"); 
    key_folder_hm.insert("deb","archive"); 

    key_folder_hm.insert("pdf","text"); 
    key_folder_hm.insert("txt","text"); 
    key_folder_hm.insert("doc","text"); 
    key_folder_hm.insert("docx","text"); 
    key_folder_hm.insert("rtf","text"); 
    key_folder_hm.insert("tex","text"); 
    key_folder_hm.insert("wpd","text"); 
    key_folder_hm.insert("odt","text"); 

    key_folder_hm.insert("stl","3d"); 
    key_folder_hm.insert("obj","3d"); 
    key_folder_hm.insert("fbx","3d"); 
    key_folder_hm.insert("dae","3d"); 
    key_folder_hm.insert("3ds","3d"); 
    key_folder_hm.insert("iges","3d"); 
    key_folder_hm.insert("step","3d"); 

    key_folder_hm.insert("pptx","presentation"); 
    key_folder_hm.insert("ppt","presentation"); 
    key_folder_hm.insert("pps","presentation"); 
    key_folder_hm.insert("key","presentation"); 
    key_folder_hm.insert("odp","presentation"); 
    
    key_folder_hm.insert("xlsx","spreadsheet"); 
    key_folder_hm.insert("xls","spreadsheet"); 
    key_folder_hm.insert("xlsm","spreadsheet"); 
    key_folder_hm.insert("ods","spreadsheet"); 

    key_folder_hm.insert("otf","font"); 
    key_folder_hm.insert("ttf","font"); 
    key_folder_hm.insert("fon","font"); 
    key_folder_hm.insert("fnt","font"); 

    key_folder_hm.insert("gif","gif"); 

    key_folder_hm.insert("exe","exe"); 

    key_folder_hm.insert("bat","bat"); 

    key_folder_hm.insert("apk","apk"); 

    for entry in WalkDir::new(&get_current_dir).min_depth(1).max_depth(1) { // directory traversal 
        let entry = entry.unwrap();
        file_path = entry.path().display().to_string();
        file_path_for_split = file_path.clone();
        file_path_for_split_slash = file_path.clone();
        
        expansion_key = split_point_slash::split_and_saving_after_point(&mut file_path_for_split); // getting the file extension

        if file_path == expansion_key  {
            println!("is directory");
        } else {
            println!("is file");
        
        let key_for_folder = key_folder_hm.get(&expansion_key as &str); // search for extensions in hash maps

        let mut string_key_for_folder: &str = "folder_not_specified";
        match key_for_folder {
        None => println!("error, the folder is not specified for this extension"),
        Some(str) => {
            string_key_for_folder = key_for_folder.unwrap();
            string_key_for_folder.to_string();
            }
        }
    
        expansion_key = split_point_slash::split_and_saving_after_slash(&mut file_path_for_split_slash); // getting the full file name
        println!("full file name: {expansion_key}");
       
        new_file_path = format!("{}\\{string_key_for_folder}\\{expansion_key}", get_current_dir.display());
        println!("new file path: {new_file_path}"); 
        
        if fs::rename(file_path, new_file_path).is_err() {
            println!("file could not be moved");
        } else {
            println!("moving the file...");
        }
    }
    }

    for entry in WalkDir::new(&get_current_dir).min_depth(1).max_depth(1) { // deleting empty folders
        if fs::remove_dir("audio").is_ok() { 
            println!("deleting an empty folder...");
        } else if fs::remove_dir("audio").is_err() {
            println!("error deleting a folder");
        }
        if fs::remove_dir("video").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("video").is_err() {             
            println!("error deleting a folder");
        }
        if fs::remove_dir("data").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("data").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("image").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("image").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("archive").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("archive").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("text").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("text").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("3d").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("3d").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("presentation").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("presentation").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("spreadsheet").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("spreadsheet").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("font").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("font").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("gif").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("gif").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("exe").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("exe").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("bat").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("bat").is_err() {
            println!("error deleting a folder");
        } 
        if fs::remove_dir("apk").is_ok() {
            println!("deleting an empty folder...");
        } else if fs::remove_dir("apk").is_err() {
            println!("error deleting a folder");
        } 
    }

}