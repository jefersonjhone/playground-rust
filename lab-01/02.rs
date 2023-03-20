use std::io;
use std::ops::Div;
use std::ops::Rem;


fn main(){
    let mut strin = String::new();
    io::stdin().read_line(&mut strin).expect("erro ao ler");
    let valor:u16 = strin.trim().parse().expect("erro ao converter para numero");
    let notas_100:u16 = valor.div(100);
    let resto_100:u16 = valor.rem(100);
    let notas_50:u16 = resto_100.div(50);
    let resto_50:u16 = resto_100.rem(50);
    let notas_10:u16 = resto_50.div(10);
    println!("{}",notas_100);
    println!("{}",notas_50);
    println!("{}",notas_10);
}