use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

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
    formula_file: Option<String>,
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // 数式文字列をスペースで区切る関数
    pub fn eval(&self, formula: &str) -> i32 {
        // rev()でVecの格納した値を逆から取り出す
        // collect()でイテレーターをコレクションに変換
        // Vec<_>の_をデータ型に置いておくことで、コンパイラが適切な型を決めてくれる
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    // 計算処理を繰り返す関数
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32{
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            }else{
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }
            // -vオプションが指定されている場合は、この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack)
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax");
        }
    }
}

// 行単位の読み込みをする関数
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

fn main() {
    let args = Args::parse();

    // 入力ファイル(formula_file)が指定されていた場合に、そのファイルパスを取り出す
    if let Some(path) = args.formula_file {
        // パスを与えることでファイルハンドルを取得する
        let f = File::open(path).unwrap();
        // 特定の単位で切り出して読み込むので、BufReaderを使用。
        let reader = BufReader::new(f);
        run(reader, args.verbose)
    }else{
        // 標準入力のハンドルを取得
        let stdin = stdin();
        // 入力をロック
        // stdin()の結果をそのまま使うと1バイト読み込みごとに排他制御が働き遅くなる
        let reader = stdin.lock();
        run(reader, args.verbose);
    }
}
