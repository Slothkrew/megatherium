mod urls;
mod wheel;

fn main() {
    let punishment = wheel::spin();
    println!("{}!", punishment);
}
