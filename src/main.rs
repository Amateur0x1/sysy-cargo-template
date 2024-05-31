
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use koopa::front::ast::Block;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);


#[derive(Debug)]
pub struct CompUnit {
  pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
  pub func_type: FuncType,
  pub ident: String,
  pub block: Block,
}

// 定义函数类型
#[derive(Debug)]
pub enum FuncType {
    Int,
    Void,
}



fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;

  // parse input file
  let ast = sysy::CompUnitParser::new().parse(&input).unwrap();
  println!("{:#?}", ast);
  Ok(())
}
