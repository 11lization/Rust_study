fn main() {
    // let number = 3;

    // //Rust������ �ݵ�� ���ǹ����� boolean Ÿ���� �;��Ѵ�.
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
    //if�� ǥ�����̹Ƿ� number�� �Ҵ��� ���� �ְ���. �ٸ� ��ȯ ���� �������� block �ȿ� ǥ������ �ʿ��ϴ�.
    //�׸��� �̷� �Ҵ��� ���ؼ��� if�� else�� ��ȯ�� Ÿ���� ��ġ�ؾ� �Ѵ�.
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}