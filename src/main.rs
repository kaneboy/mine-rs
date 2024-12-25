mod field;

use crate::field::Field;

fn main() {

    let main_field = Field::new(10, 10, 10);

    let stdout = console::Term::stdout();

    loop {
        main_field.print();
        let char = stdout.read_char().unwrap();
    }

}