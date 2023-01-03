struct PhoneModel {
    company_name: String,
    model_name: String,
    // --- snip ---
    on_sale: RefCell<bool>,
}
fn main() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        // --- snip ---
        on_sale: RefCell::new(true),
    };
    // mutability inside of something that is immutable
    super_phone_3000.replace(false);
    // Run-Time check
    // super_phone_3000.active.borrow_mut(); // first mutable borrow - okay
    // super_phone_3000.active.borrow_mut(); // second mutable borrow - not okay
}
