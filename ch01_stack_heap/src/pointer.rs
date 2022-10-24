fn main() {
    let hello = "hello world".to_string();
    println!("RODATA: {:p}", "hello world");
    println!("STACK (&hello): {:p}", &hello);
    println!("HEAP (&*hello): {:p}", &*hello);
}
