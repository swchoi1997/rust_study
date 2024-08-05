use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut cnt: i32 = 0;

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        let result: bool = game(&secret_number);

        cnt += 1;
        if result == true{
            break
        }

        if cnt > 10{
            println!("You Lose");
            println!("Secret Number : {}", secret_number);
            break
        }
    }
}
fn game(secret_number: &i32) -> bool{
    println!("Guess the number!");

    /*
        1 ~ 100 사이의 정수를 생성합니다.
        use rand::Rng; 의존성이 필요합니다.
    */

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {guess}");
    // println!("Secret_number : {secret_number}");

    guess_result(&guess, &secret_number)

}

fn guess_result(guess: &String, secret_number: &i32) -> bool{
    // String 타입과, i32타입을 비교하기 위해 형 변환을 합니다.
    // let guess_number: i32 = guess.trim()
    //     .parse()
    //     .expect("Convert Error"); // Result타입 로 체크를 해주지 않으면 빌드가 되지않음\

    let guess_number: i32 = convert_and_check_guess_is_num(&guess).unwrap_or_else(||{
        println!("Please Input Valid Number");
        println!("Your Input : {}", guess);
        std::process::exit(0);
    });

    // Ordering 이 Enum 타입이라 match에 들어가는 모든 값을 매칭 시켜줘야함
    match guess_number.cmp(secret_number){
        Ordering::Less => {
            println!("Small");
            false
        },
        Ordering::Greater => {
            println!("Big");
            false
        },
        Ordering::Equal => {
            println!("You win");
            true
        },
    }
}

fn convert_and_check_guess_is_num(guess: &String) -> Option<i32>{
    match guess.trim().parse(){
        Result::Ok(num) => Some(num),
        Result::Err(_) => None,
    }
}
