use std::fs;
use std::path::Path;

use student_management::file_handler;
use student_management::student::{self, Student};

#[test]
fn json_roundtrip() {
    let data = vec![
        Student { id: "A001".into(), name: "Alice".into(), gender: "F".into(), birth_date: "01/01/2000".into(), address: "HN".into() },
        Student { id: "A002".into(), name: "Bob".into(), gender: "M".into(), birth_date: "02/02/2001".into(), address: "HCM".into() },
    ];

    let filename = "test_students.json";
    file_handler::save_to_json(&data, filename).expect("save json");
    let loaded = file_handler::load_students_from_file(filename).expect("load json");
    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[0].id, "A001");

    let path = Path::new("data").join(filename);
    if path.exists() { let _ = fs::remove_file(path); }
}

#[test]
fn sort_by_birth_date() {
    let mut list = vec![
        Student { id: "1".into(), name: "X".into(), gender: "M".into(), birth_date: "03/01/2002".into(), address: "".into() },
        Student { id: "2".into(), name: "Y".into(), gender: "F".into(), birth_date: "01/12/2001".into(), address: "".into() },
        Student { id: "3".into(), name: "Z".into(), gender: "F".into(), birth_date: "invalid".into(), address: "".into() },
    ];
    student::sort_students(&mut list, "3");
    assert_eq!(list[0].id, "2");
    assert_eq!(list[1].id, "1");
    assert_eq!(list[2].id, "3");
}


