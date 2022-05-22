fn main() {
    
    /*
    //상수는 해당 format으로만 선언할 수 있다.
    const MAX_POINTS: u32 = 100_000;
    
    //mut를 추가하면 가변 변수를 만들 수 있다. 
    let mut x = 5;
    println!("The value of x is: {}", x);
    
    x = 6;
    println!("The value of x is: {}", x);
    */

    /*
    let x = 5;

    //1st x : shadowed by 2nd x
    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
    */

    /*
    let spaces = "     ";

    //이렇게 하면 spaces의 data type도 바꿀 수 있다. mut로 하면 spaces는 str이기 때문에 컴파일 에러가 난다.
    let spaces = spaces.len();
    
    println!("The value of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");

    let y: f32 = 3.0;

    let f: bool = false;

    let z = 'Z';
    */

    //tup에서 특정값을 빼내오기 위해서 다른 변수를 이용해 tup을 해체할 수 있다.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    //물론 index로 접근도 가능하다.

    let five_hundred = tup.0;

    println!("The value of five_hundred is: {}", five_hundred);

    //Rust에서 배열의 길이는 고정이다.(stack에 할당된다. 다른 곳에서는 data에 할당되나?)
    let a = [1, 2, 3, 4, 5];

    let first = a[0];

    //만약 index가 배열의 길이를 넘어서면 런타임 에러가 발생하는데, 다른 언어는 접근 후 예외를 던지지만 Rust에서는 접근 전에 프로그램을 종료한다.
    //let element = a[6]; 
}