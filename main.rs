const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;


fn soma (a:i32, b:i32) -> i32 {
	println! ("{} + {} = {}", a, b, a + b);
	a + b
}


fn sombra () {
	let a = 12;

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

	let mut booleana:bool = true;
	println!("Booleana = {}, tamanho booleana = {} bytes", booleana, std::mem::size_of_val(&booleana));

	let letra:&str = "C";
	println!("Tamanho do char = {} bytes", std::mem::size_of_val(&letra));
}


fn main () {
	escopo();	
	sombra();
	println!("soma = {}", soma(2,2));
	
}