use crate::{file_handler, student};
use crate::student::Student;
use std::io::{self, Write};
use std::process::Command;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("sh").args(&["-c", "clear"]).status().unwrap();
    }
}

fn pause() {
    print!("\nNhấn Enter để tiếp tục...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn display_menu() {
    println!("\n--- CHƯƠNG TRÌNH QUẢN LÝ SINH VIÊN ---");
    println!("1. Đọc danh sách từ file");
    println!("2. Thêm sinh viên");
    println!("3. Xem danh sách sinh viên");
    println!("4. Cập nhật thông tin sinh viên");
    println!("5. Xóa sinh viên");
    println!("6. Tìm kiếm sinh viên");
    println!("7. Sắp xếp danh sách");
    println!("8. Xem thống kê");
    println!("9. Lưu danh sách ra file");
    println!("0. Thoát chương trình");
    println!("---------------------------------------");
    print!("Vui lòng chọn chức năng: ");
}

fn save_data_menu(students: &Vec<Student>) {
    println!("\n--- Lưu file ---");
    println!("1. Lưu file .txt");
    println!("2. Lưu file .json");
    println!("3. Lưu file .xml");
    println!("4. Lưu file .xlsx (Excel)");
    print!("Chọn định dạng file: ");
    
    let mut choice = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => if file_handler::save_to_txt(students, "students.txt").is_ok() { println!("=> Lưu file students.txt thành công!"); },
        "2" => if file_handler::save_to_json(students, "students.json").is_ok() { println!("=> Lưu file students.json thành công!"); },
        "3" => if file_handler::save_to_xml(students, "students", "students.xml").is_ok() { println!("=> Lưu file students.xml thành công!"); },
        "4" => if file_handler::save_students_to_excel(students, "students.xlsx").is_ok() { println!("=> Lưu file students.xlsx thành công!"); },
        _ => println!("=> Định dạng không hợp lệ."),
    }
}

fn sort_menu(students: &mut Vec<Student>) {
    println!("\n--- Sắp xếp theo ---");
    println!("1. MSSV (tăng dần)");
    println!("2. Họ Tên (A-Z)");
    println!("3. Ngày sinh (tăng dần)");
    println!("0. Quay lại");
    print!("Chọn tiêu chí: ");
    
    let mut choice = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    student::sort_students(students, choice.trim());
}

pub fn run() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        clear_screen();
        display_menu();
        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                print!("Nhập tên file cần đọc (ví dụ: students.txt, data.json): ");
                io::stdout().flush().unwrap();
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                match file_handler::load_students_from_file(filename.trim()) {
                    Ok(loaded_students) => {
                        students = loaded_students;
                        println!("=> Đã đọc dữ liệu từ file '{}' thành công!", filename.trim());
                    }
                    Err(e) => println!("=> Lỗi khi đọc file: {}", e),
                }
            }
            "2" => student::add_student(&mut students),
            "3" => student::display_student_table("Danh sách sinh viên", &students),
            "4" => student::update_student(&mut students),
            "5" => student::delete_student(&mut students),
            "6" => student::search_students(&students),
            "7" => sort_menu(&mut students),
            "8" => student::show_statistics(&students),
            "9" => save_data_menu(&students),
            "0" => {
                println!("Cảm ơn đã sử dụng chương trình!");
                break;
            }
            _ => println!("=> Lựa chọn không hợp lệ, vui lòng chọn lại."),
        }
        pause();
    }
}