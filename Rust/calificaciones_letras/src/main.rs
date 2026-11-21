fn obtener_grado(calificacion: i32) -> char {
    if calificacion >= 90 && calificacion <= 100 {
        'A'
    } else if calificacion >= 80 && calificacion < 90 {
        'B'
    } else if calificacion >= 70 && calificacion < 80 {
        'C'
    } else if calificacion >= 60 && calificacion < 70 {
        'D'
    } else {
        'F'
    }
}

fn main() {
    let calificaciones = [85, 88, 90, 78, 92, 99];

    for calif in calificaciones {
        let grado = obtener_grado(calif);
        println!("Calificación del estudiante: {}, Grado: {}", calif, grado);
    }
}
