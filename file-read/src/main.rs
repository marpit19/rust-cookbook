use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    // creates a thread-local random number generator
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let test_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut test = File::new_with_data("test.txt", &test_data);

    let mut buffer: Vec<u8> = vec![];

    test = open(test).unwrap();
    let test_length = test.read(&mut buffer).unwrap();
    test = close(test).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", test);
    println!("{} is {} bytes long", &test.name, test_length);
    println!("{}", text);

    let test2 = File::new("test2.txt");

    let test2_name = test2.name();
    let test2_length = test2.len();

    println!("{:?}", test2);
    println!("{} is {} bytes long", test2_name, test2_length);
}
