fn main () {
    
    let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];

    for indice in 0..notas.len() {
        println!("A nota = {} é = {}", indice +1, notas[indice]);
    }

    matriz ();
    println!("É fim de de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    cores ();

    conteudo_opcional ()
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

#[allow(dead_code)]
enum DiaDaSemana { 
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool
{
    match dia_da_semana { 
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true, 
        _ => false 
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black:u8}
}

fn cores () {
    let cor = Color::CymkColor{cyan: 100, magenta: 50, yellow: 70, black:255};

    println!("Cor = {}", match cor {
        Color:: Red => "Vermelho",
        Color:: Green => "Verde",
        Color::Blue => "blue",
        Color::RgbColor(0,0,0)
            | Color::CymkColor{cyan:_,magenta:_,yellow:_,black:255} => "Preta",
        Color::RgbColor(_,_,_) => "RGB desconhecido",
        Color::CymkColor{cyan:_,magenta:_,yellow:_,black:_} => "CYMK desconhecido"
    });
}

fn conteudo_opcional () {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some (valor) => println!("{}", valor),
        None => println! ("ARquivo não existe")
    };

    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza de que há valor em {}", valor)
    }
}





fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
}