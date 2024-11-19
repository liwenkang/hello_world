// mod 声明：
// mod health_check 告诉 Rust 编译器存在一个名为 health_check 的模块
// 这个模块的代码可能在 health_check.rs 文件中，或者在 health_check/mod.rs 目录中
// 同理适用于 subscriptions 模块
mod health_check;
mod subscriptions;

// pub use 重导出：
// pub use 是一种重导出语法，使得外部代码可以更方便地访问这些函数
// 这样做的好处是，外部代码可以直接使用 crate::routes::health_check 而不需要写 crate::routes::health_check::health_check
// 这种模式被称为"重导出模式"，用于提供更清晰的公共 API
pub use health_check::*;
pub use subscriptions::*;