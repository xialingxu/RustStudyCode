use core::num;
use std::fmt::Display;
use std::ops::Add;

trait Pilot {
    fn fly(&self);
}

trait wizard {
    fn fly(&self);
}

trait Animal {
    fn baby_name() -> String;
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Wrapper(Vec<String>);

struct Dog;

struct Human;

extern "C" {
    // 这个函数是C语言的标准库中的函数
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//为Point实现OutlinePrint，没有实现就用默认的
impl OutlinePrint for Point {}

fn main() {
    ////////////////////////////引用裸指针///////////////////////////
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    ////////////////////////////引用不安全函数///////////////////////////
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    println!("{:?}", a);
    println!("{:?}", b);

    ////////////////////////////调用外部代码///////////////////////////
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    ////////////////////////////访问不安全的可变静态变量///////////////////////////
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    ////////////////////////////默认泛型参数///////////////////////////
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    println!("value is {:#?}", Millimeters(1000) + Meters(1));

    assert_eq!(Millimeters(1000) + Meters(1), Millimeters(2000));

    ////////////////////////////消除歧义的完全限定语法///////////////////////////

    let person = Human;
    person.fly();
    Pilot::fly(&person); //使用更加明确的方式调用

    println!("A baby dog is called a {}", Dog::baby_name());

    //完全限定<类型名 as trait名>::函数名
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    ////////////////////////////超类和子类的trait实现///////////////////////////
    //打印出来的是一个矩形
    Point { x: 1, y: 0 }.outline_print();

    ////////////////////////////使用newtype模式///////////////////////////
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    ////////////////////////////高级函数///////////////////////////

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    //下面的Vec<String>是一个泛型类型，需要手动指定类型
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    println!("{:?}", list_of_strings);
    println!("{:?}", list_of_strings2);

    //returns_closure返回的是一个闭包!!!
    let test=returns_closure();
    println!("{}",test(10));
}

fn add_one(x: i32) -> i32 {
    x + 1
}

//第一个参数是函数指针，第二个参数是i32类型。有点类似java中的Function接口
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure()->Box<dyn Fn(i32)->i32>{
    Box::new(|x|x+1)
}
