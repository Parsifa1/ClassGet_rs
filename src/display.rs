use crate::params::ValiPara;
use anyhow::Result;

pub trait SpecializedDisplay {
    fn display(self) -> Self;
}

impl SpecializedDisplay for Result<ValiPara> {
    fn display(self) -> Self {
        self.or_else(error_handler)
    }
}

impl SpecializedDisplay for Result<Vec<String>> {
    fn display(self) -> Self {
        self.map_err(error_handler).inspect(|v| {
            println!("你的课程列表为：");
            v.iter().for_each(|i| println!("{} ", i));
        })
    }
}

impl SpecializedDisplay for Result<Vec<usize>> {
    fn display(self) -> Self {
        self.map_err(error_handler).inspect(|v| {
            println!("你选择的课程为：");
            v.iter().for_each(|i| print!("{} ", i));
            println!();
        })
    }
}

fn error_handler<T>(error: anyhow::Error) -> T {
    println!("{}", error);
    std::process::exit(1);
}
