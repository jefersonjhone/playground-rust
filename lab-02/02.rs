use std::io;
fn main(){
    let mut escala = String::new();
    let mut ent2 = String::new();
    io::stdin().read_line(& mut escala).expect("erro ao ler ");
    io::stdin().read_line(& mut ent2).expect("erro ao ler ");
    let temp : f32 = ent2.trim().parse().expect("erro ao converter");
    if escala.trim() == "C"{
        let calc : f32 = temp * 9.0/5.0+32.0;
        println!("{:.2}",calc);
    }else if escala.trim() == "F"{
        let calc : f32 = 5.0/9.0 * (temp - 32.0);
        println!("{:.2}",calc);
    }
}