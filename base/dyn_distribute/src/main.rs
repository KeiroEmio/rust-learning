trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) { println!("Woof!"); }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) { println!("Meow!"); }
}

fn make_speak(animal: &dyn Animal) {
    animal.speak();
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
    for animal in animals.iter() {
        animal.speak(); // 动态分发，根据具体类型调用不同的实现
    }
make_speak(dog);
}
