fn main(){

    let variavel:i32 = 100;
    let bytes_var = std::mem::size_of_val(&variavel);
    println!("| Iniciando no Rust |\n");
    println!("1 Byte = 8 Bits");
    println!("{} | {} Bytes | {} Bits", variavel, bytes_var, bytes_var * 8);

    let decimal:f32 = 2.5;
    let bytes_decimal = std::mem::size_of_val(&decimal);
    println!("{} | {} Bytes | {} Bits", decimal, bytes_decimal, bytes_decimal * 8);

    let mut booleano:bool = true;
    booleano = false;
    let bytes_booleano = std::mem::size_of_val(&booleano);
    println!("{} | {} Bytes", booleano, bytes_booleano);

    let letra:char = 'a';
    let bytes_letra = std::mem::size_of_val(&letra);
    print!("{} | {} Bytes | {} bits", letra, bytes_letra, bytes_letra * 8);
}