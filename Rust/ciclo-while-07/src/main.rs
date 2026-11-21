fn main() {
    // Esto es para mantener corriendo el código mientras se cumpla la condición
    let mut contador = 10;

    while contador > 0 {
        println!("Conteo: {}", contador);
        contador -= 1;
    }
    println!("FIN");
}
