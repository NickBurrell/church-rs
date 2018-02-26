use std::str::FromStr;

pub fn vec_ref_to_string<T: ToString>(v: &Vec<T>) -> String {
    let mut out_str = String::from_str("").unwrap();
    for i in 0..v.len() {
        out_str.push_str(&v[i].to_string());
        if i < v.len() - 1 {
            out_str.push_str(", ");
        }
    }
    out_str
}

pub fn vec_to_string<T: ToString>(v: Vec<T>) -> String {
    let mut out_str = String::from_str("").unwrap();
    for i in 0..v.len() {
        println!("{}", out_str);
        out_str.push_str(&v[i].to_string());
        if i < v.len() - 1 {
            out_str.push_str(", ");
        }
    }
    out_str
}

