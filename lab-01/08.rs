use std::io;
fn main(){
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("erro na leitura");
    let valor :f32 = entrada.trim().parse().expect("erro na converção");
    println!("{:.3}",valor * 1.0/3.0);
}
