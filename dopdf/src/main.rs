use std::{fs, thread, time};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use regex::Regex;
use lazy_static::lazy_static;


#[derive(Debug)]
struct FileInfo {
    name: String,
    path: String,
}
fn deal_line(str: &str) -> Option<FileInfo> {
    lazy_static! {
        static ref REGETX_COMMENT: Regex = Regex::new(r"<!--(.*?)-->").unwrap();
        static ref REGETX_FILEINFO: Regex = Regex::new(r"\[([^]]+)\]\(([^)]+)\)").unwrap();
    }
    let remaining_text = REGETX_COMMENT.replace_all(str, "").trim().to_string();
    if remaining_text.is_empty() {
        None
    } else {
        REGETX_FILEINFO.captures(&*remaining_text).and_then(|cap| {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let path = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            if !name.is_empty() && !path.is_empty() {
                Some(FileInfo {
                    name: name.to_string(),
                    path: path.to_string(),
                })
            } else {
                None
            }
        })
    }
}


fn main() -> Result<(), Error> {
    // 将所有 md 文档写入一个文件
    let prefix_path = "E:\\project\\rust-course\\src\\";
    let summary_path = prefix_path.to_string() + "SUMMARY.md";
    let summary_file = File::open(summary_path)?;


    let buffered_reader = BufReader::new(summary_file);

    let mut i = 1;
    for line in buffered_reader.lines() {
        let mut buffered_writer = BufWriter::new(
            OpenOptions::new().create(true).append(true).open("./out".to_string() + &*(i /20  + 1).to_string() + ".md")?);
        // 去除字符串中的注释
        let unwrapped_line = line.unwrap();
        match deal_line(unwrapped_line.as_str()) {
            None => {}
            Some(file_info) => {
                match fs::read_to_string(prefix_path.to_string() + &*file_info.path){
                    Ok(str) => {
                        println!("写入第{}部分", i);
                        buffered_writer.write_all(str.as_bytes())?;
                        buffered_writer.write_all("<div style=\"page-break-after: always;\"></div>\n".as_bytes())?;
                        buffered_writer.flush()?;
                        thread::sleep(time::Duration::from_millis(10));
                        i = i + 1;
                    }
                    Err(_) => {}
                }
            }
        };
    }

    Ok(())
}
// fn deal_line(str: &str) -> Option<&str> {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(r"<!--(.*?)-->").unwrap();
//     }
//     RE.captures(str).and_then(|cap| cap.get(1).map(|comment| comment.as_str()))
// }

