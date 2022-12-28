fn main () {
	let variavel:u8 = 128;
	println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
	
	let decimal:f32 = 2.5;
	println!("decimal = {}", decimal);

	let mut booleana:bool = true;
	println!("Booleana = {}, tamanho booleana = {} bytes", booleana, std::mem::size_of_val(&booleana));

	let letra:str = "C";
	println!("Tamanho do char = {} bytes", std::mem::size_of_val(&letra));
}