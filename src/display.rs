pub trait SpecializedDisplay {
    fn display(self) -> Self;
}

impl SpecializedDisplay for String {
    fn display(self) -> Self {
        self
    }
}

impl SpecializedDisplay for Vec<String> {
    fn display(self) -> Self {
        println!("你的课程列表为：");
        self.iter().for_each(|i| print!("{} ", i));
        self
    }
}

impl SpecializedDisplay for Vec<usize> {
    fn display(self) -> Self {
        println!("你选择的课程为：");
        self.iter().for_each(|i| print!("{} ", i));
        self
    }
}

pub fn error_handler<T>(error: anyhow::Error) -> anyhow::Result<T> {
    println!("{}, 请重启程序", error);
    std::process::exit(1); // 退出程序
}
