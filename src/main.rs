const PI:f32 = 3.141592653;
static mut GLOBAL:u8 = 255;

fn main(){
    println!("| Rust |\n");
    
    let result_hello_world:String = hello_world("");
    println!("{}", result_hello_world);
    
    types();
    
    let result_soma:String = soma(10, 20);
    println!("{}", result_soma);
}

pub fn types(){
    println!("| Types |\n");
    let variavel:i32 = 100;
    let bytes_var = std::mem::size_of_val(&variavel);
    println!("i32 | {} | {} Bytes | {} Bits", variavel, bytes_var, bytes_var * 8);
    
    let decimal:f32 = 2.5;
    let bytes_decimal = std::mem::size_of_val(&decimal);
    println!("f32 | {} | {} Bytes | {} Bits", decimal, bytes_decimal, bytes_decimal * 8);
    
    let booleano:bool = true;
    let bytes_booleano = std::mem::size_of_val(&booleano);
    println!("bool | {} | {} Bytes", booleano, bytes_booleano);
    
    let letra:char = 'a';
    let bytes_letra = std::mem::size_of_val(&letra);
    println!("char | {} | {} Bytes | {} bits\n", letra, bytes_letra, bytes_letra * 8);
    
    println!("PI = {}\n", PI);
    unsafe{
        println!("Global = {}", GLOBAL);
    }
}

pub fn hello_world(nome:&str) -> String{
    println!("| Hi! |\n");
    if !nome.is_empty() {
        return format!("Hello World, {}!\n", nome);
    }
    
    format!("Hello World!\n")
}

pub fn soma(x:i128, y:i128) -> String {
    println!("| Soma |\n");
    format!("{} + {} = {}", x, y, x + y)
}