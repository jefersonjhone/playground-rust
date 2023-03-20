use std::io;
use std::ops::Rem;
fn main(){
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("erro ao ler");
    let num : i16 = number.trim().parse().expect("erro ao converter");
    let new :i16 = num.rem(100);
    if num.rem(new) == 0{
        println!("SIM");
    }else{
        println!("NAO");
    }
}