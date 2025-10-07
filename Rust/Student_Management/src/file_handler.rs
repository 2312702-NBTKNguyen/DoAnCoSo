use crate::student::Student;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use rust_xlsxwriter::{Format, FormatAlign, Workbook};

fn read_from_txt(filename: &str) -> io::Result<Vec<Student>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut students = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 5 {
            students.push(Student {
                id: parts[0].to_string(),
                name: parts[1].to_string(),
                gender: parts[2].to_string(),
                birth_date: parts[3].to_string(),
                address: parts[4].to_string(),
            });
        }
    }
    Ok(students)
}

fn read_from_json<T: DeserializeOwned>(filename: &str) -> io::Result<Vec<T>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(data)
}

pub fn load_students_from_file(filename: &str) -> io::Result<Vec<Student>> {
    match Path::new(filename).extension().and_then(|s| s.to_str()) {
        Some("txt") => read_from_txt(filename),
        Some("json") => read_from_json(filename),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Định dạng file không được hỗ trợ (chỉ .txt và .json).",
        )),
    }
}

pub fn save_to_txt(students: &Vec<Student>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for student in students {
        writeln!(file, "{},{},{},{},{}", student.id, student.name, student.gender, student.birth_date, student.address)?;
    }
    Ok(())
}

pub fn save_to_json<T: Serialize>(data: &Vec<T>, filename: &str) -> io::Result<()> {
    let json_string = serde_json::to_string_pretty(data).unwrap();
    let mut file = File::create(filename)?;
    file.write_all(json_string.as_bytes())
}

pub fn save_to_xml<T: Serialize>(data: &Vec<T>, root_element: &str, filename: &str) -> io::Result<()> {
    let mut xml_string = String::new();
    xml_string.push_str(&format!("<{}>\n", root_element));
    for item in data {
        if let Ok(item_xml) = quick_xml::se::to_string(item) {
            xml_string.push_str("  ");
            xml_string.push_str(&item_xml);
            xml_string.push('\n');
        }
    }
    xml_string.push_str(&format!("</{}>\n", root_element));
    
    let mut file = File::create(filename)?;
    file.write_all(xml_string.as_bytes())
}

pub fn save_students_to_excel(students: &Vec<Student>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let header_format = Format::new().set_bold().set_align(FormatAlign::Center);

    worksheet.write_string_with_format(0, 0, "MSSV", &header_format)?;
    worksheet.write_string_with_format(0, 1, "Họ Tên", &header_format)?;
    worksheet.write_string_with_format(0, 2, "Giới tính", &header_format)?;
    worksheet.write_string_with_format(0, 3, "Ngày sinh", &header_format)?;
    worksheet.write_string_with_format(0, 4, "Địa chỉ", &header_format)?;

    for (i, student) in students.iter().enumerate() {
        let row = (i + 1) as u32;
        worksheet.write_string(row, 0, &student.id)?;
        worksheet.write_string(row, 1, &student.name)?;
        worksheet.write_string(row, 2, &student.gender)?;
        worksheet.write_string(row, 3, &student.birth_date)?;
        worksheet.write_string(row, 4, &student.address)?;
    }

    worksheet.autofit();
    workbook.save(filename)?;
    Ok(())
}