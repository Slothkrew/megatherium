mod urls;

fn main() {
    let own_name = urls::get_name();
    println!("Hello from {}!", own_name);
}
