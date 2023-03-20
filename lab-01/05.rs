use std::io;
fn main(){
    let altura : f32 = 3.0;
    let comprimento : f32 = 12.0;
    let material : f32 = 100.0;
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("erro na entrada");
    let servico : f32 = entrada.trim().parse().expect("erro ao converter");
    let custo:f32 = altura * comprimento * servico + material;
    println!("{}",custo);

}