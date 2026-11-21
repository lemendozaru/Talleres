fn main() {
    let temperaturas = [25.2, 26.5, 27.4, 24.4, 25.2, 26.8, 24.9];
    let mut suma = 0.0;

    for temperatura in temperaturas {
        suma += temperatura;
    }
    let temp_promedio = suma / temperaturas.len() as f32;
    println!("El total semanal es {}", suma);
    println!("La temperatura promedio es {}", temp_promedio);
}
