mod ch1;
mod ch2;
mod ch3;
mod ch9;
fn main() {
    println!("第一章: Hello World");
    ch1::display();
    ch1::testcase_list();
    ch1::format();
    println!("第二章: 原生类型");
    ch2::tuples();
    println!("第三章：结构体");
    ch3::structs();
    ch3::main();
    println!("第九章：函数");
    ch9::std_examples();
    ch9::hof_example();
}
