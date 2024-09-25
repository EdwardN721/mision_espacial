use std::collections::HashMap;
use std::io;
use std::io::Write;
use rand::Rng;

fn regresar(posicion: usize, recursos: i32, integridad_nave: i32) -> bool {
    if posicion == 2 { // Tierra
        println!("***************************************");
        let mut input = String::new();
        print!("¿Deseas regresar a la Tierra? (S/N): ");
        input.clear();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Por favor, digita un valor válido!");
        let regreso = input.trim().to_uppercase();
        println!("\n=======================================\n");
        if regreso == "S" {
            println!("Has regresado con éxito!");
            println!("Has extraído: {} recursos en tu última expedición!", recursos);
            println!("Tu nave tiene el {} de integridad", integridad_nave);
            return true;
        } else {
            println!("La misión debe continuar!");
        }
    }
    false
}

fn extraer_recursos(recurso_planeta: &mut i32, recurso_nave: i32) -> i32 {
    let recurso_maximo = *recurso_planeta;
    let random = rand::thread_rng().gen_range(0..recurso_maximo + 1);
    let extraccion = recurso_maximo - random;

    if extraccion > *recurso_planeta {
        *recurso_planeta = 0; // Asegurarse de no dejar recursos negativos
    } else {
        *recurso_planeta -= extraccion; // Disminuir el recurso en el planeta
    }

    println!("La nave extrajo: {} recursos | Carga de la nave: {}", extraccion, recurso_nave + extraccion);
    recurso_nave + extraccion // Retornar el nuevo total de recursos en la nave
}

fn evento(integridad_nave: &mut i32, recursos_nave: &mut i32) -> bool {
    let evento_tipo = rand::thread_rng().gen_range(0..2);
    let danio_integridad = rand::thread_rng().gen_range(0..30);

    if evento_tipo == 0 {
        return false; // Sin evento
    }

    *integridad_nave -= danio_integridad;
    if *integridad_nave <= 15 {
        println!("LA NAVE ENTRA EN ESTADO CRÍTICO, DEBE VOLVER A LA TIERRA");
    }

    let perdida_recursos = rand::thread_rng().gen_range(10..=*recursos_nave);
    *recursos_nave = (*recursos_nave).saturating_sub(perdida_recursos); // Asegurar no tener un valor negativo
    println!("La nave tuvo dificultades y ha perdido {} recursos y {} de integridad.", perdida_recursos, danio_integridad);
    println!("===Cargamento actualizado: {}===", recursos_nave);
    true
}

fn mover(avanzar: &String, posicion: &mut usize, planeta_lista: [&str; 9]) {
    match avanzar.as_str() {
        "I" => {
            if *posicion == 0 {
                println!("¡Ya no podemos avanzar, no vamos a quemar con el sol!");
            } else {
                *posicion -= 1; // Avanza a la izquierda
            }
        }
        "D" => {
            if *posicion == planeta_lista.len() - 1 {
                println!("¡Ya no podemos avanzar, no sabemos qué hay más allá!");
            } else {
                *posicion += 1; // Avanza a la derecha
            }
        }
        _ => {
            println!("Dirección no válida. Por favor, ingresa 'I' o 'D'.");
        }
    }
}

fn solicitar_direccion(input: &mut String) -> bool {
    print!("¿Hacia qué dirección deseas avanzar? (I/D): ");
    input.clear();
    io::stdout().flush().unwrap();
    match io::stdin().read_line(input) {
        Ok(_) => true,
        Err(e) => {
            println!("Error al leer la entrada: {}", e);
            false
        }
    }
}

fn main() {
    let mut planetas_recuros = HashMap::from([
        ("Mercurio", 500),
        ("Venus", 800),
        ("Tierra", 0),
        ("Marte", 1000),
        ("Jupiter", 1500),
        ("Saturno", 1200),
        ("Urano", 700),
        ("Neptuno", 600),
        ("Pluton", 200),
    ]);

    const PLANETAS_LISTA: [&str; 9] = [
        "Mercurio", "Venus", "Tierra", "Marte", "Jupiter", "Saturno", "Urano", "Neptuno", "Pluton"
    ];

    println!("¡Estás en una misión espacial!");
    let mut posicion = 2; // Comenzamos en Tierra
    let mut input = String::new();
    let mut recursos_nave = 0;
    let mut integridad_nave = 100;

    loop {
        let planeta_actual = PLANETAS_LISTA[posicion];
        let recurso_planeta = match planetas_recuros.get_mut(planeta_actual) { // Usar get_mut para modificar el valor
            Some(recurso) => recurso,
            None => {
                println!("Error: El planeta {} no se encontró.", planeta_actual);
                continue; // Regresar al inicio del ciclo si hay un error
            }
        };
        println!("Planeta: {} | Recursos: {}", planeta_actual, recurso_planeta);

        if !solicitar_direccion(&mut input) {
            continue;
        }

        mover(&input.trim().to_uppercase(), &mut posicion, PLANETAS_LISTA);

        //Logica recursos y eventos
        if *recurso_planeta > 0 { // Verifica si hay recursos para extraer
            recursos_nave = extraer_recursos(recurso_planeta, recursos_nave);
            if evento(&mut integridad_nave, &mut recursos_nave) {
                println!("---Salud de la nave: {}!---", integridad_nave);
            } else {
                println!("La salida del planeta fue exitosa!");
            }
        } else {
            println!("Ya No hay recursos en este planeta.");
        }

        if integridad_nave <= 0 {
            println!("Tu nave ha sido destruida. Fin del juego.");
            break;
        }

        if regresar(posicion, recursos_nave, integridad_nave) {
            break;
        } else {
            println!("\n-----------------\n");
        }
    }
}
