pub fn split_and_saving_after_point (str: &mut String) -> String {
    let mut s: String = String::from("");
    for x in str.chars() {
        if x == '.' {
            s = "".to_string();
        } else {
            s += &x.to_string().as_str();
        } 
    }
    *str = s; 
    return str.to_string();
}

pub fn split_and_saving_after_slash (str: &mut String) -> String {
    let mut s: String = String::from("");
    for x in str.chars() {
        if x == '\\' {
            s = "".to_string();
        } else {
            s += &x.to_string().as_str();
        } 
    }
    *str = s; 
    return str.to_string();
}
