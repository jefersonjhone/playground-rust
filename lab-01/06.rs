use std::io;
use std::ops::Div;
use std::ops::Rem;

fn main(){
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("erro na conversao");
    let valor : u64 = entrada.trim().parse().expect("erro na conversao");
    let milhar : u64 = valor.div(1000);
    let centena : u64 = valor.rem(1000).div(100);
    let dezena : u64 = valor.rem(1000).rem(100).div(10);
    let unidade : u64 = valor.rem(1000).rem(100).rem(10);
    println!("{}",milhar + centena + dezena + unidade);
}