fn main() {
    let v1:Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3 = Vec::new();
    v3.push(5);
    {
        let v1 = vec![1, 2, 3, 4];

    }
    
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);
    match v4.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    
    let mut v5 = vec![1, 2, 3, 4, 5];
    let first = &v5[0];
    println!("The first element is: {}", first);
    v5.push(6);
    for i in &v5 {
        println!("{}", i);
    }

    for i in &mut v5 {
        *i += 50;
    }

//    let does_not_exist = &v4[100];
//    let does_not_exist = v4.get(100);
}
