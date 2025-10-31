use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::collections::HashMap;
use std::io::{self, Write};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub gender: String,
    pub birth_date: String,
    pub address: String,
}

pub fn display_student_table(title: &str, students: &[Student]) {
    println!("\n--- {} ---", title.to_uppercase());
    if students.is_empty() {
        println!("Không có dữ liệu để hiển thị.");
        return;
    }

    let mut max_id = "MSSV".len();
    let mut max_name = "Họ Tên".len();
    let mut max_gender = "Giới tính".len();
    let mut max_birth = "Ngày Sinh".len();
    let mut max_addr = "Địa Chỉ".len();

    for s in students {
        max_id = max(max_id, s.id.len());
        max_name = max(max_name, s.name.len());
        max_gender = max(max_gender, s.gender.len());
        max_birth = max(max_birth, s.birth_date.len());
        max_addr = max(max_addr, s.address.len());
    }

    let line = format!(
        "+-{:-<id_w$}-+-{:-<name_w$}-+-{:-<gender_w$}-+-{:-<birth_w$}-+-{:-<addr_w$}-+",
        "", "", "", "", "",
        id_w = max_id, name_w = max_name, gender_w = max_gender, birth_w = max_birth, addr_w = max_addr
    );

    println!("{}", line);
    println!(
        "| {:<id_w$} | {:<name_w$} | {:<gender_w$} | {:<birth_w$} | {:<addr_w$} |",
        "MSSV", "Họ Tên", "Giới tính", "Ngày Sinh", "Địa Chỉ",
        id_w = max_id, name_w = max_name, gender_w = max_gender, birth_w = max_birth, addr_w = max_addr
    );
    println!("{}", line);

    for s in students {
        println!(
            "| {:<id_w$} | {:<name_w$} | {:<gender_w$} | {:<birth_w$} | {:<addr_w$} |",
            s.id, s.name, s.gender, s.birth_date, s.address,
            id_w = max_id, name_w = max_name, gender_w = max_gender, birth_w = max_birth, addr_w = max_addr
        );
    }
    println!("{}", line);
}

pub fn add_student(students: &mut Vec<Student>) {
    println!("\n--- Thêm sinh viên mới ---");
    let mut id = String::new();
    let mut name = String::new();
    let mut gender = String::new();
    let mut birth_date = String::new();
    let mut address = String::new();

    print!("Nhập MSSV: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).expect("Failed to read line");

    print!("Nhập Họ tên: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    print!("Nhập Giới tính: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut gender).expect("Failed to read line");

    print!("Nhập Ngày sinh (dd/mm/yyyy): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut birth_date).expect("Failed to read line");

    print!("Nhập Địa chỉ: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut address).expect("Failed to read line");

    let birth = birth_date.trim();
    if NaiveDate::parse_from_str(birth, "%d/%m/%Y").is_err() {
        println!("=> Ngày sinh không hợp lệ. Định dạng đúng: dd/mm/yyyy");
        return;
    }

    let new_student = Student {
        id: id.trim().to_string(),
        name: name.trim().to_string(),
        gender: gender.trim().to_string(),
        birth_date: birth.to_string(),
        address: address.trim().to_string(),
    };
    students.push(new_student);
    println!("=> Đã thêm sinh viên thành công!");
}

pub fn update_student(students: &mut Vec<Student>) {
    println!("\n--- Cập nhật thông tin sinh viên ---");
    print!("Nhập MSSV của sinh viên cần cập nhật: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id = id.trim();

    if let Some(student) = students.iter_mut().find(|s| s.id == id) {
        println!("Nhập thông tin mới (để trống nếu không muốn thay đổi):");

        let mut name = String::new();
        print!("Họ tên mới (hiện tại: {}): ", student.name);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        if !name.trim().is_empty() {
            student.name = name.trim().to_string();
        }

        // Tương tự cho các trường khác...

        println!("=> Cập nhật thành công!");
    } else {
        println!("=> Không tìm thấy sinh viên với MSSV này.");
    }
}

pub fn delete_student(students: &mut Vec<Student>) {
    println!("\n--- Xóa sinh viên ---");
    print!("Nhập MSSV của sinh viên cần xóa: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id = id.trim();

    let initial_len = students.len();
    students.retain(|s| s.id != id);

    if students.len() < initial_len {
        println!("=> Đã xóa sinh viên thành công!");
    } else {
        println!("=> Không tìm thấy sinh viên với MSSV này.");
    }
}

pub fn search_students(students: &[Student]) {
    print!("Nhập từ khóa tìm kiếm (MSSV, Tên hoặc Địa chỉ): ");
    io::stdout().flush().unwrap();
    let mut query = String::new();
    io::stdin().read_line(&mut query).expect("Failed to read line");
    let query = query.trim().to_lowercase();

    let results: Vec<Student> = students
        .iter()
        .filter(|s| {
            s.id.to_lowercase().contains(&query)
                || s.name.to_lowercase().contains(&query)
                || s.address.to_lowercase().contains(&query)
        })
        .cloned()
        .collect();

    display_student_table("Kết quả tìm kiếm", &results);
}

pub fn sort_students(students: &mut Vec<Student>, choice: &str) {
    match choice {
        "1" => students.sort_by(|a, b| a.id.cmp(&b.id)),
        "2" => students.sort_by(|a, b| a.name.cmp(&b.name)),
        "3" => students.sort_by(|a, b| {
            let da = NaiveDate::parse_from_str(&a.birth_date, "%d/%m/%Y");
            let db = NaiveDate::parse_from_str(&b.birth_date, "%d/%m/%Y");
            match (da.ok(), db.ok()) {
                (Some(da), Some(db)) => da.cmp(&db),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        }),
        _ => return,
    }
    println!("=> Đã sắp xếp danh sách thành công!");
    display_student_table("Danh sách sau khi sắp xếp", students);
}

pub fn show_statistics(students: &[Student]) {
    println!("\n--- THỐNG KÊ SINH VIÊN ---");
    if students.is_empty() {
        println!("Chưa có dữ liệu để thống kê.");
        return;
    }

    let total = students.len();
    println!("- Tổng số sinh viên: {}", total);

    let mut gender_counts: HashMap<String, i32> = HashMap::new();
    for s in students {
        *gender_counts.entry(s.gender.trim().to_string()).or_insert(0) += 1;
    }
    println!("- Theo giới tính:");
    for (gender, count) in gender_counts {
        println!("  + {}: {} sinh viên", gender, count);
    }
}