fn main() {
    let nombre = String::from("Alicia");
    saludando(nombre);
    let suma = calcular_suma(5, 10);
    println!("El resultado es {}", suma);
}

fn saludando(nombre: String) {
    println!("Hola {}, bienvenido.", nombre)
}

fn calcular_suma(a: i32, b: i32) -> i32 {
    let resultado = a + b;
    resultado
}
