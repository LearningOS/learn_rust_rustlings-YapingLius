// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references


fn main() {
    // 字符串切片到字符串
    let data = "Rust is great!".to_string();

    //let datas = data.clone();
    get_char(&data);

   string_uppercase( data.to_string());
}
//只有在确定字符串中至少有一个字符时，才使用unwrap.
// Should not take ownership
fn get_char(data: &String) -> char {

    data.chars().last().unwrap()
}
// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}