pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    //动态数组，存储实现了Draw特性的对象
    pub components: Vec<Box<dyn Draw>>,
}

//这个结构体代码的位置也是很重要的，放在下面会报错
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}



impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
    }
}

// 使用泛型来定义结构体并实现trait约束
// pub struct Screen<T:Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T:Draw {
//         pub fn run(&self) {
//             for component in self.components.iter(){
//                 component.draw();
//             }
//         }
//     }