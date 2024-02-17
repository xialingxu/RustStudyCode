use ch17::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 实际绘制选择框的代码
    }
    
}

fn main() {
    let mut x=AveragedCollection::new();
    x.add(1);
    x.add(2);
    x.add(3);
    x.add(4);
    x.add(5);
    println!("average:{}",x.average());


    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(Button {
                width: 40,
                height: 20,
                label: String::from("CANCEL"),
            }),
            Box::new (SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            })
        ],
    };
    screen.run();



}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

//通过结构体的方法来实现各种操作
impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
 
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}