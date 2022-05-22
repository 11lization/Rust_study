fn main() {
    // let number = 3;

    // //Rust에서는 반드시 조건문으로 boolean 타입이 와야한다.
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }


    
    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }



    let condition = true;
    //if는 표현식이므로 number에 할당할 수도 있겠지. 다만 반환 값은 가지도록 block 안에 표현식이 필요하다.
    //그리고 이런 할당을 위해서는 if와 else의 반환값 타입이 일치해야 한다.
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}