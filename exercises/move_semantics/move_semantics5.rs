// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// 共有アクセスは読み出しのみのアクセス
// let xとした場合はxの値を変更不可
// 可変アクセスは排他アクセス
// yがxを参照する生存期間でzはxを参照することができない
#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
