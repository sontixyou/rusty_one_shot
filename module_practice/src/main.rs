mod hoge_calculator;
use module_practice::number_calculator;
// use crate::number_calculator;でも行けそうだが、だめだった。理由はよくわかっていない。同じcrate内だから、使えると思ったが。

fn main() {
    let a = 10;
    let b = 20;
    let result = number_calculator::add(a, b);
    println!("{} + {} = {}", a, b, result); // 10 + 20 = 30

    let c = 30;
    let d = 40;
    let another_result = hoge_calculator::another_number_calculator::add(c, d);
    println!("{} + {} = {}", c, d, another_result); // 30 + 40 = 70
    let another_nested_result =
        hoge_calculator::another_number_calculator::nested_another_number_calculator::sub(c, d);
    println!("{} - {} = {}", c, d, another_nested_result); // 30 - 40 = -10
}
