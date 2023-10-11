fn main() {

    let a: i32 = 10;
    let b: u16 = 20;
    if a < (b as i32) {
        println!("10 < 20");
    }

    try_in();

    // assert!(0.1 + 0.2 == 0.3); // panic Error， thread 'main' panicked at 'assertion failed: 0.1 + 0.2 == 0.3'

    // cf();

    epsi();

    // is_nan();
    nnan();
}

use std::convert::TryInto;
fn try_in() {
    let a: i32 = 10;
    let b: u16 = 20;

    // 这里用 unwrap 简单处理一下，实际项目要好好处理
    let b = b.try_into().unwrap();
    if a < b {
        println!("try_into 10 < 20");
    }
}

fn cf() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    // to_bits 改为 16 进制
    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits()); // 3e99999a
    println!("        0.3: {:x}", abc.2.to_bits());           // 3e99999a
    println!();


    println!("xyz (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits()); // 3fd3333333333334
    println!("        0.3: {:x}", xyz.2.to_bits());           // 3fd3333333333333
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2); // panic 



}

fn epsi() {
    let result: f32 = 0.1 + 0.2;
    let desired: f32 = 0.3;
    let abs_diff = (desired - result).abs();
    println!("f32 abs_diff = {}", abs_diff);
    assert!(abs_diff <= f32::EPSILON);
    println!("f32 desired <= f32::EPSILON");

    let result: f64 = 0.1 + 0.2;
    let desired: f64 = 0.3;
    let abs_diff = (desired - result).abs();
    println!("f64 abs_diff = {}", abs_diff);
    assert!(abs_diff <= f64::EPSILON);
    println!("f64 desired <= f64::EPSILON");

}

fn is_nan() {
    // 计算 -42.0 的平方根
    let x = (-42.0_f32).sqrt();
    assert_eq!(x, x); // Panic
}

fn nnan() {
    let x: f32 = 1.0 / 0.0;
    println!("nnan x is {}", x); // inf
    assert!(x.is_finite());
}