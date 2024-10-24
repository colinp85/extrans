use extrans::ido::{Ido};

fn main() {
    let mut d = Ido::new();
    let mut e: Ido = Ido::new();

    e.set_string(&100, String::from("nested"));
    e.set_string(&101, String::from("nested_two"));

    d.set_string(&1, String::from("Blah"));
    d.set_string(&2, String::from("More Blah"));

    d.append_array(&3, e);  // nested

    d.set_integer(&4, 32);

    println!("{}", d.to_string());
}