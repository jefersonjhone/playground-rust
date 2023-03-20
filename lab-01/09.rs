use std::io;
use std::f64::consts::PI;
fn main(){
    let mut pontos = String::new();
    let mut dano_1 = String::new();
    let mut dano_2 = String::new();
    io::stdin().read_line(&mut pontos).expect("erro na leitura");
    io::stdin().read_line(&mut dano_1).expect("erro na leitura");
    io::stdin().read_line(&mut dano_2).expect("erro na leitura");
    let ponts : f64 = pontos.trim().parse().expect("erro de conversao");
    let dano1 : f64 = dano_1.trim().parse().expect("erro de conversao");
    let dano2: f64 = dano_2.trim().parse().expect("erro de conversao");
    let result : f64 = ponts - ((5.0 * dano1).powf(1.0/2.0)+ PI.powf(dano2/3.0)).floor();
    println!("{:.2}",result)
    
}