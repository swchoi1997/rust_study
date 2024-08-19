#![allow(unused_variables)]

type Message = String;

#[derive(Debug)]
struct Mailbox{
    mailbox: Vec<Message>,
}

impl Mailbox{
    fn receive(&mut self, msg: Message) -> Result<String, String> {
        self.mailbox.push(msg);

        Ok(String::from("Success"))
    }
}

#[derive(Debug)]
struct CubeSat{
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat{
    fn new(_id: u64) -> CubeSat{
        CubeSat{
            id: _id,
            mailbox: Mailbox{mailbox: Vec::new()}
        }
    }

    fn send_message(&self, receiver: &mut CubeSat, msg: Message) -> Result<String, String>{
        let message = format!("Send to : {} -> {}", self.id.to_string(), msg);

        receiver.mailbox.receive(message)
    }
}

#[derive(Debug)]
enum StatusMessage{
    OK,
}
fn check_status(sat_id: CubeSat) -> CubeSat{
    println!("{:?}: {:?}", sat_id, StatusMessage::OK);
    sat_id
}


fn main() {
    let mut sat_a = CubeSat::new(100);
    let mut sat_b = CubeSat::new(200);
    let mut sat_c = CubeSat::new(300);


    if sat_a.send_message(&mut sat_b, String::from("Test")).is_err(){
        println!("ERROR")
    }

    if sat_a.send_message(&mut sat_c, String::from("Test")).is_err(){
        println!("ERROR")
    }

    dbg!(&sat_a);
    dbg!(&sat_b);
    dbg!(&sat_c);

}