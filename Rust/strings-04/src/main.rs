fn main() {
    // Hay dos tipos de strings en Rust

    // Uno inmutable que es &str
    let saludo: &str = "Hola a todos";
    println!("{}", saludo);

    // que también puede declararse sin tipado:
    let nombre = "Juan";
    println!("Hola, {}", nombre);

    // Y el segundo tipo sí es mutable
    let mut modelo = String::from("Nissan");
    modelo.push_str(" Tsuru");
    println!("{}", modelo);
}
