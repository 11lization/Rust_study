fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    //Rust에서는 이런 식을 구문이라고 부른다. 구문은 반환값이 없고, block에서 반환값을 나타낼 때 쓰이지 않는다.
    //x + 1;

    //대신에 끝에 ;을 제거함으로써 이것이 반환값임을 나타낸다.
    x + 1
}

//참고 : Rust에서는 let x = (let y = 6);과 같은 식이 작동하지 않는다. 왜냐하면 let y = 6은 구문이므로 아무런 반환값이 없기 때문이다.