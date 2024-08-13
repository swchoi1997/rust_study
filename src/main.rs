#![allow(unused_variables)]

static mut ERROR: i32 = 0;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File{
    fn new(name: &str) -> File{
        File{
            name: String::from(name),
            data: Vec::new()
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File{
        File{
            name: String::from(name),
            data: data.clone()
        }
    }

    fn read(&mut self, save_to: &mut Vec<u8>) -> usize {
        self.data.append(save_to);

        self.data.len()
    }
}

fn open(f: &mut File) -> bool{
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &mut File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp:Vec<u8> = f.data.clone();
    let tmp_size = tmp.len();

    save_to.reserve(tmp_size);
    save_to.append(&mut tmp);

    tmp_size
}

fn main() {
    let mut f2 = File {
        name: String::from("t2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };


    let mut buffer: Vec<u8> = vec![];

    if open(&mut f2){
        let read_size: usize = read(&mut f2, &mut buffer);

        close(&mut f2);
    }

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{:?}", text);


    let mut f3: File = File::new("t3.txt");

    println!("{:?}, {:?}", &f3.name, &f3.data);

    open(&mut f3);
    unsafe {
        if ERROR != 0 {
            panic!("When Read File Occured Error")
        }
    }
    let size: usize = f3.read(&mut vec![1, 2, 3, 4, 5]);
    close(&mut f3);
    unsafe {
        if ERROR != 0 {
            panic!("When Read File Occured Error")
        }
    }

    println!("{size}")






}