use rand::{thread_rng,Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let mut rng = thread_rng();

    let n1: u8 = rng.gen();
    let f1: f64 = rng.gen();

    println!("{}",n1);
    println!("{}",f1);

    println!("{}",rng.gen::<u32>());
    println!("{}",rng.gen::<i32>());

    let mut random_range = rand::thread_rng();

    let int = random_range.gen_range(0..=200);
    let fl = random_range.gen_range(0.00..=200.00);
    println!("{}",int);
    println!("{}",fl);

    let string_random = create_random_alphanum(100);
    println!("{}",string_random);
    create_random_password(80);
}

fn create_random_alphanum(amount: usize) -> String {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(amount)
        .map(char::from)
        .collect();
    format!("{}",random_string)
}

fn create_random_password(password_size: usize) -> () {
    const DEFINED_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                     abcdefghijklmnopqrstuvwxyz\
                                     0123456789)(*&^%$#@!~";
    let LENGTH: usize = password_size;
    let mut range = thread_rng();

    let new_password: String = (0..=LENGTH)
        .map(|_| {
            let index = range.gen_range(0..DEFINED_CHARSET.len());
            DEFINED_CHARSET[index] as char
        })
        .collect();
    println!("{}",new_password);
}


