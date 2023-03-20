use std::io;
use std::f64::consts::PI;

fn main(){
    let mut entrada = String::new();
    io::stdin().read_line(& mut entrada).expect("deu erro na leitura");
    let valor :f64 = entrada.trim().parse().expect("erro na concersao");
    let circ:f64 = (4.0/3.0) * PI * valor.powf(3.0);
    let area:f64 = PI * valor.powf(2.0);
    println!("{:.2}",area);
    println!("{:.2}",circ);
}