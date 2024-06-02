use std::collections::HashMap;
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

pub trait GenerateIR {
    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>>;
}

impl GenerateIR for CompUnit {
    fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        // 定义函数返回类型的映射关系
        let mut type_mapping: HashMap<FuncType, String> = HashMap::new();
        type_mapping.insert(FuncType::Int, String::from("i32"));
        type_mapping.insert(FuncType::Void, String::from("()"));

        // 转换函数定义
        let func_def = format!(
            "fun @{}(): {} {{\n",
            self.func_def.ident,
            type_mapping.get(&self.func_def.func_type).ok_or("Unknown function type")?
        );

        // 转换块内容
        let mut block_content = String::new();
        block_content.push_str("%entry:\n");
        block_content.push_str(&format!("  ret {}", self.func_def.block.stmt.num));

        // 返回转换后的 Rust 代码
        Ok(format!("{}{}\n}}", func_def, block_content))
    }
}
