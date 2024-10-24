use std::time::SystemTime;
use extrans::ido::Ido;

fn elapsed_time(start: SystemTime, end: SystemTime) -> u128 {
    let elapsed = end.duration_since(start).expect("time went backwards");
    elapsed.as_micros()
}

fn test_strings(iters: i32) {
    let mut ido = Ido::new();
    let mut vals: Vec<String> = Vec::<String>::new();

    for num in 0..iters {
        vals.push(num.to_string());
    }

    let start = SystemTime::now();
    let mut count = 0;
    for x in vals {
        ido.set_string(&count, x);
        count += 1;
    }
    let end = SystemTime::now();
    let elapsed = elapsed_time(start, end);
    let op: f64 = f64::from(elapsed as u32) / f64::from(iters);

    println!("{:<20}: {}, {:.3}", "test_strings", elapsed, op);
}

fn test_integers(iters: i32) {
    let mut ido = Ido::new();
    let mut vals: Vec<i64> = Vec::<i64>::new();

    for num in 0..iters {
        vals.push(num as i64);
    }

    let start = SystemTime::now();
    let mut count = 0;
    for x in vals {
        ido.set_integer(&count, x);
        count += 1;
    }

    let end = SystemTime::now();
    let elapsed = elapsed_time(start, end);
    let op: f64 = f64::from(elapsed as u32) / f64::from(iters);

    println!("{:<20}: {}, {:.3}", "test_integers", elapsed, op);
}

fn main() {
    let iters = 10000;
    test_strings(iters);
    test_integers(iters);
}