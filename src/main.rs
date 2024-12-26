mod field;

use crate::field::Field;

fn main() {

    println!("\n--- 方向键移动，空格键探索 ---\n");

    let mut main_field = Field::new(10, 10, 10);

    loop {
        main_field.print();

        let key = console::Term::stdout().read_key().unwrap();

        if key == console::Key::ArrowUp     { main_field.move_cursor(-1, 0); }
        if key == console::Key::ArrowDown   { main_field.move_cursor(1,  0); }
        if key == console::Key::ArrowLeft   { main_field.move_cursor(0, -1); }
        if key == console::Key::ArrowRight  { main_field.move_cursor(0,  1); }

        if key == console::Key::Char(' ') {
            if !main_field.open() {
                println!("\n--- Oops, 踩雷！---\n");
                break;
            }
            if main_field.all_opened() {
                println!("\n--- 干得漂亮，已找到所有地雷！---\n");
                break;
            }    
        }        

        // 将当前光标向上移动N行。
        print!("\x1b[{}A", main_field.rows);
    }

}