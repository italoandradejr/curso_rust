fn main () {
    
    let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];

    for indice in 0..notas.len() {
        println!("A nota = {} é = {}", indice +1, notas[indice]);
    }

    matriz ();
}

fn matriz () {
    let matriz: [[f32; 3]; 2] = [
        [0.0,1.2,0.1],
        [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}