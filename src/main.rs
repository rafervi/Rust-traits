use std::ops::Add;
#[allow(unused_assignments)]
#[allow(unused_variables)]
struct RustDev {
    awesome: bool
}
struct JavaDev {
    awesome: bool
}
trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {println!("Hello World!")}
}
impl Developer for RustDev{
    fn new(awesome: bool) -> Self {
        RustDev{awesome: awesome}
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello World!\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev{awesome: awesome}
    }

    fn language(&self) -> &str {
        "Java 1.8"
    }

    fn say_hello(&self) {
        println!("System.out.println(\"Hello World!\");");
    }
}
//Generics
trait Bark {
    fn bark(&self) -> String;
}

/*struct Dog {
    species: &'static str
}
struct Cat {
    color: &'static str
}
impl Bark for Dog {
    fn bark(&self) -> String {
     //   return format!("{} barking", self.species);
    }
}*/
fn bark_it<T: Bark>(b:T){
    println!("{}", b.bark());
}
struct Dog {}
struct Cat {}
//Animal trait cannot be return because of Rust Memory Warranties
trait Animal {
    fn make_noise(&self) -> &'static str;
}
impl Animal for Dog{
    fn make_noise(&self) -> &'static str {
        "Woof"
    }
}
impl Animal for Cat {
    fn make_noise(&self) -> &'static str {
        "Meow"
    }
}
fn get_animal(rand_number: f64) -> Box<dyn Animal> {
        if rand_number < 1.0 {
            Box::new(Dog{})
        } else {
            Box::new(Cat{})
        }
}
trait Summable<T> {
    fn sum(&self) -> T;
}
 impl Summable<i32> for Vec<i32>{
     fn sum(&self) -> i32 {
        let mut sum = 0;
         for i in self{
                sum+=*i;
         }
         sum
     }
 }
//overload operators
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y

        }
    }
}
trait Duplicateable {
    fn dupl(&self) -> String;
}
impl Duplicateable for String{
    fn dupl(&self) -> String {
        format!("{0}{0}",*self)
    }
}

impl Duplicateable for i32 {
    fn dupl(&self) -> String {
        format!("{}",*self * 2)
    }
}
/*fn duplicate<T: Duplicateable> (x:T){ //Monomorphization
    println!("{}",x.dupl());
}*/
fn duplicate(x: &dyn Duplicateable){
    println!("{}", x.dupl());
}


fn main() {
    //let r = RustDev{awesome: true};
    let r = RustDev::new(true);
    let j = JavaDev::new(false);
    println!("{}",r.language());
    r.say_hello();
    println!("{}",j.language());
    j.say_hello();
    //let dog = Dog{species: "retriever"};
    //let cat = Cat{color:"black"};
   // bark_it(dog);
    println!("The animal says {}",get_animal(0.5).make_noise());
    println!("The animal says {}",get_animal(2.0).make_noise());
    let a = vec![1, 2, 3, 4, 5];
    println!("Sum = {}", a.sum());
    let p1 = Point {x:1.3, y:4.6};
    let p2 = Point {x: 3.7, y: 1.4};
    let p3 = p1+p2;
    println!("{:?}",p3);
    let a = 42;
    let b = "Hi John ".to_string();
    //duplicate(a);
    //duplicate(b);
    duplicate(&a);
    duplicate(&b);

}
