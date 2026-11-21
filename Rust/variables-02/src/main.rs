fn main() {
    // Variable inmutable
    let valor = 9;
    println!("El valor es {}", valor);

    // Variable mutable
    let mut valor02 = 5;
    valor02 = 10;
    println!("El nuevo valor es {}", valor02);

    // Sombrear variables
    let valor03 = 64;
    let valor03 = valor03 + 1;
    println!("Variable valor03 cambió a {}", valor03);

    // Cambiar tipo
    let valor03 = "una cadena";
    println!("Ahora es {}", valor03);
}
