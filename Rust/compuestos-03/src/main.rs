fn main() {
    // Los arrays contienen datos del mismo tipo
    // su tamaño se fija al declararse
    let numeros = [1, 2, 3, 4, 5];
    // Se accede a sus elementos como sigue:
    println!("Elemento en el lugar 0 es {}", numeros[0]);

    // Las tuplas contienen datos de distinto tipo
    let persona = ("Alicia", 30, 5.4);
    println!("Nombre: {}", persona.0);
    println!("Edad: {}", persona.1);
    println!("Estatura: {}", persona.2);

    // Las constantes no pueden cambiar ni sombrear sus valores
    const PI: f32 = 3.14;
    println!("El valor de pi es {}", PI);
}
