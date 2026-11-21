fn main() {
    let mut contador = 0;
    let mut a = 0;
    let mut b = 1;

    loop {
        if contador == 10 {
            break;
        }
        if contador == 0 {
            println!("{}", a);
        } else if contador == 1 {
            println!("{}", b);
        } else {
            let siguiente = a + b;
            println!("{}", siguiente);
            a = b;
            b = siguiente;
        }
        contador += 1;
    }
}
