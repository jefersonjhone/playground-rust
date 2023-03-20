use std::io;
fn main(){
    let mut soma = 0.0;
    for c in 1..5{
        let mut notas = String::new();
        io::stdin().read_line(&mut notas).expect("erro na leitura");
        let nota:f32 = notas.trim().parse().expect("erro ao converter");
        soma += nota * c as f32;
    }
    println!("{:.2}",soma/10.0)
}