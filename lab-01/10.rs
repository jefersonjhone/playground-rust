use std::io;
fn main(){
    let mut xa = String::new();
    let mut ya = String::new();
    let mut xb = String::new();
    let mut yb = String::new();
    io::stdin().read_line(&mut xa).expect("erro na entrada");
    io::stdin().read_line(&mut ya).expect("erro na entrada");
    io::stdin().read_line(&mut xb).expect("erro na entrada");
    io::stdin().read_line(&mut yb).expect("erro na entrada");
    let ax : f64= xa.trim().parse().expect("erro de converção");
    let ay : f64 = ya.trim().parse().expect("erro de converção");
    let bx : f64 = xb.trim().parse().expect("erro de converção");
    let by : f64 = yb.trim().parse().expect("erro de converção");
    let disx = bx - ax;
    let disy = by - ay;
    let dis = (disx.powf(2.0) + disy.powf(2.0)).sqrt();
    println!("{:.3}",dis)
}