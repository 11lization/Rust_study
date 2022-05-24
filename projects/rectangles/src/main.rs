// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// fn main() {
//     let rect1 = Rectangle { length: 50, width: 30 };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.length * rectangle.width
// }



// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// fn main() {
//     let rect1 = Rectangle { length: 50, width: 30 };

//     println!("rect1 is {:?}", rect1);
// }



#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // 연관함수. self를 통해 작동하는 것은 아니지만 해당 구조체를 반환할 수 있다.
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}