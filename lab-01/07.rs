use std::io;
fn main(){
    let mut entrada = String::new();
    io::stdin().read_line(& mut entrada).expect("erro na leitura");
    let vendas: f32 = entrada.trim().parse().expect("erro na converção");
    let lucro: f32 = vendas * 0.3;
    println!("{:.2}",lucro);
}