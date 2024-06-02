use koopa::ir::{entities::Value, ValueKind};
use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use std::env::args;
use std::io::Result;
use std::fs::{self, read_to_string, write};
use ast::*;
// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);
pub mod ast;

trait GenerateAsm<T> {
    fn generate(&self, value_id: Value) -> Result<T>;
}

impl GenerateAsm<()> for koopa::ir::Program {
    fn generate(&self, value_id: Value) -> Result<()> {
        for &func in self.func_layout() {
            let _ = self.func(func).generate(value_id)?;
        }
        Ok(())
    }
}

impl GenerateAsm<i32> for koopa::ir::FunctionData {
    fn generate(&self, value_id: Value) -> Result<i32> {
        let value_use = self.dfg().values();
        let data = value_use.get(&value_id).unwrap();
        match data.kind() {
            ValueKind::Integer(int) => {
                let value = int.value();
                Ok(value)
            }
            _ => unreachable!(),
        }
    }
}

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
  let koopa_code = ast.to_string().unwrap();
  // 写入输出文件
  let driver = koopa::front::Driver::from(koopa_code);
  let program = driver.generate_program().unwrap();
  let mut riscv_code = String::new();
  riscv_code.push_str("  .text\n");
  riscv_code.push_str("  .globl main\n");
  riscv_code.push_str("main:           # @main\n");
  riscv_code.push_str("    li a0, "); // 加载立即数到寄存器 a0
  for &func in program.func_layout() {
    let func_data = program.func(func);
    for (&bb, node) in func_data.layout().bbs() {
      for &inst in node.insts().keys() {
        let value_data_node = func_data.dfg().value(inst);
        match value_data_node.kind() {
          ValueKind::Integer(_int) => {
            //
          }
          ValueKind::Return(ret) => {

            let value_id = ret.value().unwrap();
            let value =func_data.generate(value_id).unwrap();
            riscv_code.push_str(&value.to_string());
            riscv_code.push_str("\n");
            riscv_code.push_str("    ret\n");
          }
          // 其他种类暂时遇不到
          _ => unreachable!(),
        }
        // 访问指令
        // ...
      }
    }
  }
  fs::write(output, riscv_code);

  Ok(())
}