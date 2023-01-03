use std::cell::Cell;

struct PhoneModel {
    company_name: String,
    model_name: String,
    // --- snip ---
    on_sale: Cell<bool>,
}

fn main() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        // --- snip ---
        on_sale: Cell::new(true),
    };

    super_phone_3000.on_sale.set(false); // mutability inside of something that is immutable
}
