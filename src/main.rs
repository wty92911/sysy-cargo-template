use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::{read_to_string, File};
use std::io::{Result, Write};
use koopa::back::KoopaGenerator;
// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy

lalrpop_mod!(sysy);


fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let mode = args.next().unwrap();
  println!("mode: {}", mode);

  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;

  // 调用 lalrpop 生成的 parser 解析输入文件
  let ast = sysy::CompUnitParser::new().parse(&dbg!(input)).unwrap();

  let program = ast.into();

  // convert to text form
  let mut gen = KoopaGenerator::new(Vec::new());
  gen.generate_on(&program)?;
  let text_form_ir: String = std::str::from_utf8(&gen.writer()).unwrap().to_string();
  // println!("{}", text_form_ir);
  let mut file = File::create(output)?;

  write!(file, "{}", text_form_ir)?;
  Ok(())
}
