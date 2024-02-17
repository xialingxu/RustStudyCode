fn main() {
    ///////////////////////////////////// if let  /////////////////////////////////////
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "42".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age < 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    ///////////////////////////////////// while let  /////////////////////////////////////
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    ///////////////////////////////////// for  /////////////////////////////////////

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    ///////////////////////////////////// let  /////////////////////////////////////

    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);

    ///////////////////////////////////// 模式语法  /////////////////////////////////////
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), //不可失败的模式
    }

    match x {
        1 | 2 => println!("one or two"), //或与模式
        3..=10 => println!("three"),
        _ => println!("anything"),
    }

    let a = Some(10);
    let b = 10;
    match a {
        Some(50) => println!("Got 50"),
        Some(n) if n == b => println!("Matched, n = {}", n), //带条件的模式
        _ => println!("Default case, a = {:?}", a),
    }

    let c = 'c';
    match c {
        'a'..='j' => println!("early ASCII letter"), //char也可以使用区间模式
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //////////////////结构 结构体

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    //let Point{x,y}=p;下面的代码等价于这行代码
    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x), //模式中的变量名与结构体中的字段名相同
        Point { x: 0, y } => println!("On the y axis at {}", y), //只匹配x=0，y=任意值
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    //////////////////结构 枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    //let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::Quit;

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }

    ////////////////////结构 嵌套的结构体和枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg2 = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg2 {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            );
        }
        _ => (), //不关心其他情况
    }

    ////////////////////结构中的所有权
    let s = Some(String::from("Hello"));
    if let Some(_s) = s {
        //s的所有权被移动到_s,所以s不能再使用。可以直接用Some(_)来避免s的所有权被移动
        println!("found a string");
    }
    //println!("{:?}",s); //error: value moved here

    ////////////////////..忽略值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    ////////////////////@绑定
    #[derive(Debug)]
    enum Message3 {
        Hello { id: i32 },
    }
    let msg3 = Message3::Hello { id: 5 };
    
    match msg3 {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable);
        }
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message3::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }

    println!("{:?}",msg3); //这里的值不涉及所有权，所以msg3还可以继续使用
   
}
