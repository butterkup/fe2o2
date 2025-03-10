fn greet_world() {
    let southern_germary = "Grub Gott";
    let japan = "Ching Chong Lee";
    let usa = "Hello World";

    let regions = [southern_germary, usa, japan]; // Array literal

    for &region in regions.iter() {
        println!("Region: {}", &region);
    }
}

fn main1() {
    let penguin_data = "\
    common name,length (cm)
    Little Penguin,33
    Yellow Eyed Penguin,65
    Fiordland Penguin, 60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        } 

        let name = fields[0];
        let maybe_length: Result<f32, _> = fields[1].parse();

        if maybe_length.is_err() {
            continue;
        }

        let length = maybe_length.unwrap();
        println!("{}, {}cm", name, length);
    }
}

#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat
}

fn main2() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    //drop(grains);
    println!("Grains: {:?}", grains);
}

//use std::thread;
//
//fn main3() {
//    let mut data = 100u32;
//
//    thread::spawn(move || { data = 500; });
    //thread::spawn(move || { data = 1000; });
//}

fn main4() {
    let fruit = vec!["kiwi", "banana", "blueberries"];
    let buffer_overflow = fruit[3];
    println!("{:?}", buffer_overflow);
    assert_eq!(buffer_overflow, "melon");
}

fn main() {
    // greet_world();
    // main1();
    // main2();
    // main3();
    main4();
}
