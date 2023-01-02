const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;


fn soma (a:i32, b:i32) -> i32 {
	println! ("{} + {} = {}", a, b, a + b);
	a + b
}


fn sombra () {
	let _a = 12;

	{
		let b = 456;
		println! ("dentro, b = {}", b);
	}
}

fn escopo () {

println!("PI = {}", PI);

	unsafe {
	println!("variavel_global = {}", GLOBAL);
}

	let variavel:i32 = 300;
	println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
	
	let decimal:f32 = 2.5;
	println!("decimal = {}", decimal);

	let booleana:bool = true;
	println!("Booleana = {}, tamanho booleana = {} bytes", booleana, std::mem::size_of_val(&booleana));

	let letra:&str = "C";
	println!("Tamanho do char = {} bytes", std::mem::size_of_val(&letra));
}


fn main () {
	escopo();	
	sombra();
	println!("soma = {}", soma(2,2));
	condicionais()
}

fn condicionais () {
	let idade: u8 = 17;
	let responsavel_autorizou = true;
	let eh_maior = idade >= 18;

	if eh_maior {
		println!("pode entrar")
	} else if idade > 16 && responsavel_autorizou{
		println!("Pode entrar, responsavel autorizou")
	} else {
		println!("Não pode entrar!!!");
	}

	let condicao = if eh_maior {"maior"} else {"menor"};
	println!("É {} de idade", condicao);
}
