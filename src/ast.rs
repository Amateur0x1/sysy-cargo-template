
use std::{hash::{Hash, Hasher}};
// 定义函数类型
#[derive(Debug)]
pub enum FuncType {
    Int,
    Void,
}
// 定义语句
#[derive(Debug)]
pub struct Stmt {
    pub num: i32,
}

// 定义代码块
#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}

// 定义函数定义
#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

// 定义编译单元
#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

// 手动实现 PartialEq trait
impl PartialEq for FuncType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

// 手动实现 Eq 和 Hash trait
impl Eq for FuncType {}

impl Hash for FuncType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
