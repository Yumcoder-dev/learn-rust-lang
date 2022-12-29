fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    // --snip--
}

struct Config {
    query: String,
    file_path: String,
}
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone(); // TODO: reduce runtime cost using Iterator next()
    let file_path = args[2].clone();

    Config { query, file_path }
}
