#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}

fn main() {
    let person = Person {
        name: String::from("张三"),
        age: 25
    };
    println!("{:#?}", person);  // 输出: Person { name: "张三", age: 25 }
}


// error used
// struct Person {
//     name: String,
//     age: u32
// }

// fn main() {
//     let person = Person {
//         name: String::from("张三"),
//         age: 25
//     };
//     println!("{}", person);  // 输出: Person { name: "张三", age: 25 }
// }