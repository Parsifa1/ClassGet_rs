use anyhow::Result;
pub trait SpecializedDisplay {
    fn display(self) -> Self;
}

impl SpecializedDisplay for Result<String> {
    fn display(self) -> Self {
        self.or_else(error_handler)
    }
}

impl SpecializedDisplay for Result<Vec<String>> {
    fn display(self) -> Self {
        self.map_err(error_handler).map(|v| {
            println!("你的课程列表为：");
            v.iter().for_each(|i| print!("{} ", i));
            v
        })
    }
}

impl SpecializedDisplay for Result<Vec<usize>> {
    fn display(self) -> Self {
        self.map_err(error_handler).map(|v| {
            println!("你选择的课程为：");
            v.iter().for_each(|i| print!("{} ", i));
            v
        })
    }
}

fn error_handler<T>(error: anyhow::Error) -> T {
    println!("{}, 请检查配置文件，并重启程序", error);
    std::process::exit(1);
}
