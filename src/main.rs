use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
enum FileState{
    OPEN,
    CLOSE,
}

impl Display for FileState{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::OPEN => write!(f, "OPEN"),
            FileState::CLOSE => write!(f, "CLOSE"),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for File{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File{
    fn new(name: &str) -> File {
        File{
            name: String::from(name),
            data: Vec::new(),
            state: FileState::CLOSE
        }
    }

    fn read(&mut self, save_to: &mut Vec<u8>) -> Result<usize, String>{
        if self.state != FileState::OPEN{
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp: Vec<u8> = Vec::new();
        let mut original = self.data.clone();

        let read_length: usize = original.len() + save_to.len();
        tmp.reserve(read_length);
        tmp.append(&mut original);
        tmp.append(save_to);

        self.data = tmp;


        Ok(read_length)
    }

    fn open(mut self) -> Result<File, String>{
        self.state = FileState::OPEN;

        Ok(self)
    }

    fn close(mut self) -> Result<File, String>{
        self.state = FileState::CLOSE;

        Ok(self)
    }
}

fn main() {
    {
        let mut f1: File = File::new("t1.txt");

        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(12);
        buffer.push(13);
        buffer.push(14);

        if f1.read(&mut buffer).is_err(){
            dbg!("Error checking is working");
        }

        f1 = match f1.open() {
            Ok(file) => file,
            Err(e) => {
                panic!("Error : {:?}", e);
            },
        };
        println!("{:?}", f1);
        println!("{}", f1);

        let i = f1.read(&mut buffer).unwrap();

        f1 = match f1.close() {
            Ok(file) => file,
            Err(e) => panic!("Error : {:?}", e),
        };

        println!("{} is {} bytes long", &f1.name, i);

        println!("------------------------------");

        println!("{:?}", f1);
        println!("{}", f1);


    }

}