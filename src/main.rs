extern crate rand;

use std::io;
use rand::Rng;
// 「match」での比較に使用
use std::cmp::Ordering;

fn main() {

    // 1-101 までの整数を生成する
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // 数を当ててごらん
    // ちなみに、println!() のような「!」が付くものは、関数じゃなくて「マクロ」なんだって！
    println!("Guess the number!");

    // 無限ループする
    loop {
        // ほら、予想を入力してね
        println!("Please input your guess.");

        // let は変数を作ることを意味している
        // mut はその変数が可変（代入可能）であることを意味している省略は非可変）
        // 「::」は String型の静的メソッドを使用することを意味している（関連関数というらしい）
        let mut guess = String::new();

        // 標準出力から入力を読み取って、guess に格納する（当然、guess は代入可能であることが必要）
        // 「&」は guess の参照を渡すことを意味すし、「&mut」とするのは参照も代入可能であることを示している
        io::stdin().read_line(&mut guess)
            // 行の読み込みに失敗しました
            // read_lineは処理結果として io::Result を返す。エラーの場合 expect()でプログラムをクラッシュさせる
            .expect("Failed to read line");

        // 通常であればユーザ入力の guess の最後には改行が入る。trim()して削除する。
        // parse()で型変換を行う。「u32」と記載しているため、非負整数の32bit数値に変換される。
        let guess: u32 = match guess.trim().parse() {
            // Ok の中にないっているものをそのまま返す
            Ok(num) => num,
            // この continue は loop にかかっている。loopをやり直すという意味。
            Err(_) => continue,
        };

        // 次のように予想しました: {}
        println!("You guessed: {}", guess);

        // 「cmp」で比較し、戻り値「std::cmp::Ordering」を得る
        // match はその戻り値が、{}内のどれに一致しているかを判定する
        match guess.cmp(&secret_number) {
            // 小さすぎ！
            Ordering::Less => println!("Too small!"),
            //大きすぎ！
            Ordering::Greater => println!("Too big!"),
            //やったね！
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
