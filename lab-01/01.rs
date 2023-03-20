use std::io;
fn main() {
    let mut comprimento = String::new();
    let mut altura = String::new();
    let mut preco_p_metro = String::new();
    
    io::stdin().read_line(&mut altura).expect("Erro ao ler a entrada");
    io::stdin().read_line(&mut comprimento).expect("Erro ao ler a entrada");
    io::stdin().read_line(&mut preco_p_metro).expect("Erro ao ler a entrada");
    
    
    let altura:f64 = altura.trim().parse().expect("erro ao converter altura em numero");
    let comprimento:f64 = comprimento.trim().parse().expect("erro ao converter altura em numero");
    let preco_p_metro:f64 = preco_p_metro.trim().parse().expect("erro ao converter altura em numero");
    let resultado:f64 = altura * comprimento * preco_p_metro;

    println!("{:.2} ",resultado);
}