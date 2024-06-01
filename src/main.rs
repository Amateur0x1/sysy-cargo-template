use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use std::env::args;
use std::io::Result;
use std::fs::{read_to_string, write};
use ast::*;
// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);
pub mod ast;

fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let _mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;
  // parse input file

  let ast = sysy::CompUnitParser::new().parse(&input).unwrap();
  let rust_code = ast_to_rust(ast);

  let driver = koopa::front::Driver::from(rust_code);
  let program = driver.generate_program().unwrap();
  for &func in program.func_layout() {
    let func_data = program.func(func);
    for (&bb, node) in func_data.layout().bbs() {
      for &inst in node.insts().keys() {
        let value_data = func_data.dfg().value(inst);
        match value_data.kind() {
          ValueKind::Integer(int) => {
            // 处理 integer 指令
            // ...
          }
          ValueKind::Return(ret) => {
            // 处理 ret 指令
            // ...
          }
          // 其他种类暂时遇不到
          _ => unreachable!(),
        }
        // 访问指令
        // ...
      }
    }

  }
  Ok(())
}
// 将 AST 转换为 Rust 代码的函数
pub fn ast_to_rust(ast: CompUnit) -> String {
    // 定义函数返回类型的映射关系
    let mut type_mapping: HashMap<FuncType, String> = std::collections::HashMap::new();
    type_mapping.insert(FuncType::Int, String::from("i32"));
    type_mapping.insert(FuncType::Void, String::from("()"));

    // 转换函数定义
    let func_def = format!("fun @{}(): {} {{\n", ast.func_def.ident, type_mapping[&ast.func_def.func_type]);

    // 转换块内容
    let mut block_content = String::new();
    block_content.push_str("%entry:\n");
    block_content.push_str(&format!("  ret {}", ast.func_def.block.stmt.num));
    println!("{}{}\n}}", func_def, block_content);


    // 返回转换后的 Rust 代码
    format!("{}{}\n}}", func_def, block_content)
}

