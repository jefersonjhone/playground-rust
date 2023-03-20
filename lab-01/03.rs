use std::io;
use std::ops::Div;
use std::ops::Rem;

fn main(){
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("deu erro na entrda");
    let valor : u16 = entrada.trim().parse().expect("erro ao converter");
    let notas_50 : u16 = valor.div(50);
    let resto_50 : u16 = valor.rem(50);
    let notas_10 : u16 = resto_50.div(10);
    let resto_10 : u16 = resto_50.rem(10);
    let notas_2 : u16 = resto_10.div(2);
    println!("{}",notas_50);
    println!("{}",notas_10);
    println!("{}",notas_2);
}