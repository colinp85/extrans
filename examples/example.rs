use extrans::FIX;

fn main () {
    let mut fix = FIX::new();
    let res = fix.load_dictionary("FIXT11.xml");
    match res {
        Ok(_) => {},
        Err(err) => { println!("{}", err.to_string())}
    }
}