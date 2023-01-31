//region 11.1.库
// 创建一个库，然后看看如何把它链接到另一个 crate。
pub fn public_function() {
    println!("called rary's 'public_function()'");
}
fn private_function() {
    print!("called rary's 'private_function()'");
    println!();
}

pub fn indirect_access() {
    print!("called rary's 'indirect_access()', that is ");
    private_function();
}
