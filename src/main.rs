use std::ops::RangeInclusive;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

enum ComportIpAddr {
    V4(String),
    V6(String),
}

enum ComportIpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message{
    QUIT,
    MOVE{x : i32, y: i32},
    WRITE(String),
    ChangeColor(u8, u8, u8)

}
fn main() {
    {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            addr: String::from("127.0.0.1"),
        };
    }

    {
        let home = ComportIpAddr::V4(String::from("127.0.0.1"));
        let loop_back = ComportIpAddr::V6(String::from("::1"));
    }

    {
        let home = ComportIpAddr2::V4(127, 0, 0, 1);
        let loop_back = ComportIpAddr2::V6(String::from("::1"));
    }

    {
        let quit: Message = Message::QUIT;
        dbg!(&quit);

        let _move: Message = Message::MOVE { x: 10, y: 32 };
        dbg!(&_move);

        let write: Message = Message::WRITE(String::from("HELLO"));
        dbg!(&write);

        let change_color: Message = Message::ChangeColor(255, 255, 255);
        dbg!(&change_color);
    }

    {
        let dice: i32 = rand::thread_rng().gen_range(1..=6);
        match dice {
            1 => println!("hello {}", dice),
            2 => println!("hello {}", dice),
            3 => println!("hello {}", dice),
            4 => println!("hello {}", dice),
            5 => println!("hello {}", dice),
            6 => println!("hello {}", dice),
            other=> panic!("gg")
        }

        let inclusive:RangeInclusive<i32> = 1..=7;
        loop {
            let mut dice2: i32 = rand::thread_rng().gen_range(inclusive.clone());
            sleep(Duration::from_millis(500));
            match dice2 {
                1 => println!("hello {}", dice2),
                2 => println!("hello {}", dice2),
                3 => println!("hello {}", dice2),
                4 => println!("hello {}", dice2),
                5 => println!("hello {}", dice2),
                6 => println!("hello {}", dice2),
                _ => {
                    println!("QUIT {dice2} ");
                    break;
                },
            }
        }

    }
}