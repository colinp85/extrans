//use extrans::FIX;
use extrans::properties::PropertiesBuilder;

fn main () {
    match PropertiesBuilder::new().build() {
        Ok(props) => {
            let val = props.get::<String>("mama.properties.fix_dictionary").unwrap_or("FIX42.xml".to_string());
            let vala = props.get_default::<String>("mama.properties.fix_dictionar", "FIX42.xml".to_string());
            println!("VAL2: {}", val);
            println!("VALa: {}", vala);

            /*for (key, val) in props.m_properties.collect().iter() {
                println!("Key: {} Val: {}", key, value);

            }*/
        }
        Err(_e) => {

        }
    };

    /*let mut fix = FIX::new();
    let res = fix.load_dictionary("FIX50SP2.xml");
    match res {
        Ok(_) => {},
        Err(err) => { println!("{}", err.to_string())}
    }*/
}