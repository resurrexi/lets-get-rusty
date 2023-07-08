struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool,
}

enum ProductCategory {
    Books,
    Clothing,
    Electronics,
}

// enum variants can hold data
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        String::from("JSON")
    }
}

fn main() {
    let category = ProductCategory::Electronics;
    let product = Product {
        name: String::from("TV"),
        category,
        price: 200.98,
        in_stock: true,
    };

    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("test"));
    let cmd3 = Command::MoveCursor(22, 0);
    let cmd4 = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    // using implementation of enum
    let json_string = cmd1.serialize();
}
