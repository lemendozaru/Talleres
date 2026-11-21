fn main() {
    // Este ciclo no tiene condición de fin,
    // debemos especificarlo manualmente
    let mut index = 1;

    loop {
        index += 1;
        println!("Index {}", index);

        if index == 100 {
            println!("Máximo alcanzado");
            break;
        }
    }
}
