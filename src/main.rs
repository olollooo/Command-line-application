use anyhow::{bail, ensure, Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // 数式文字列をスペースで区切る関数
    pub fn eval(&self, formula: &str) -> Result<i32> {
        // rev()でVecの格納した値を逆から取り出す
        // collect()でイテレーターをコレクションに変換
        // Vec<_>の_をデータ型に置いておくことで、コンパイラが適切な型を決めてくれる
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    // 計算処理を繰り返す関数
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {

            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            }else{
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("incalid token as {}", pos),
                };
                stack.push(res);
            }
            // -vオプションが指定されている場合は、この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack)
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Args {
    /// Sets the level of verbpsity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}

fn main() -> Result<()>{
    let args = Args::parse();

    // 入力ファイル(formula_file)が指定されていた場合に、そのファイルパスを取り出す
    if let Some(path) = args.formula_file {
        // パスを与えることでファイルハンドルを取得する
        let f = File::open(path)?;
        // 特定の単位で切り出して読み込むので、BufReaderを使用。
        let reader = BufReader::new(f);
        run(reader, args.verbose)
    } else {
        // 標準入力のハンドルを取得
        let stdin = stdin();
        // 入力をロック
        // stdin()の結果をそのまま使うと1バイト読み込みごとに排他制御が働き遅くなる
        let reader = stdin.lock();
        run(reader, args.verbose)
    }
}

// 行単位の読み込みをする関数
fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);
        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert!(calc.eval("").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());

    }
}