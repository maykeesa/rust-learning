//const PI:f32 = 3.141592653;
static mut GLOBAL:u8 = 255;

fn main(){
    println!("| Rust |\n");
    let x:i128 = 4;
    let y:i128 = 4;

    let result_hello_world:String = hello_world("Mayke");
    println!("{}", result_hello_world);

    //types();

    let soma:i128 = soma(x, y);
    println!("{} + {} = {}", x, y, soma);

    let mult:i128 = multiplicacao(x, y);
    println!("{} * {} = {}", x, y, mult);

    let idade:u8 = 9;
    let faixa_etaria:String = verificar_idade(idade);
    println!("Faixa etária: {}", faixa_etaria);

    repeticoes(5);

    let match_statement = match_statement("dfgfgdf");
    println!("Nome: {}", match_statement);

    let match_statment_number:String = match_statement_number(21);
    println!("Valor: {}", match_statment_number);
}

pub fn types(){
    println!("| Types |\n");
    let variavel:i32 = 100;
    let bytes_var:usize = std::mem::size_of_val(&variavel);
    println!("i32 | {} | {} Bytes | {} Bits", variavel, bytes_var, bytes_var * 8);
    
    let decimal:f32 = 2.5;
    let bytes_decimal:usize = std::mem::size_of_val(&decimal);
    println!("f32 | {} | {} Bytes | {} Bits", decimal, bytes_decimal, bytes_decimal * 8);
    
    let booleano:bool = true;
    let bytes_booleano:usize = std::mem::size_of_val(&booleano);
    println!("bool | {} | {} Bytes", booleano, bytes_booleano);
    
    let letra:char = 'a';
    let bytes_letra:usize = std::mem::size_of_val(&letra);
    println!("char | {} | {} Bytes | {} bits", letra, bytes_letra, bytes_letra * 8);
    
    //println!("PI = {}\n", PI);

    unsafe{
        println!("Global = {}", GLOBAL);
    }
}

pub fn hello_world(nome:&str) -> String{
    if !nome.is_empty() {
        return format!("Hello World, {}!\n", nome);
    }

    "Hello World!\n".to_string()
}

pub fn soma(x:i128, y:i128) -> i128 {
    println!("| Soma |");
    let soma:i128 = x + y;

    soma
}

pub fn multiplicacao(x:i128, y:i128) -> i128 {
    println!("| Multiplicação |");
    let mult:i128 = x * y;

    mult
}

pub fn verificar_idade(idade:u8) -> String{
    let faixa_etaria:String= if idade >= 18 { "Adulto" }else if idade >= 10 && idade < 18{ "Adolescente" }else{ "Criança" }.to_string();

    faixa_etaria
}

pub fn repeticoes(multiplicador:u8){
    let mut contador:u8 = 0;

    println!("| Tabela de {} |", multiplicador);

    //while
    while contador < 10{
        contador += 1;

        if contador == 5{
           continue;
        }

        let resultado:u8 = multiplicador * contador;
        println!("{} * {} = {}", multiplicador, contador, resultado);
    }

    //loop = while true
    contador = 0;
    loop {
        contador += 1;
        let resultado:u8 = multiplicador * contador;
        println!("{} * {} = {}", multiplicador, contador, resultado);

        if contador == 10{
            break;
        }
    }

    //for range
    for i in 1..11{
        println!("Index: {}", i);
    }
}

pub fn match_statement(opcao:&str) -> String{
    return match opcao {
        x if x.is_empty() => {
            "Nome vazio".to_string()
        },
        x if x.eq("Mayke") =>{
            "Nome e igual a Mayke".to_string()
        },
        x if x.len() > 6 => {
            "Nome muito grande!".to_string()
        },
        _ => {
            "Nenhuma das opcoes foram verdadeiras!".to_string()
        }
    };
}

pub fn match_statement_number(number:u8) -> String{
    return match number {
        1 => format!("1 = {}", number),
        2 | 3 | 5 | 7 => format!("Primo = {}", number),
        10..=20 => format!("10 <= {} <= 20", number),
        _ => "Nenhuma das opcoes foram válidas!".to_string(),
    };
}
