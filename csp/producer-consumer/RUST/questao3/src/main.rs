use std::thread;
use std::sync::mpsc::channel;
use std::io;
fn main() {

//recebe a entrada do usuario
let mut numeroDeProdutores = 10;
println!("Digite o numero de Produtores:");
	let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => numeroDeProdutores = i,
        Err(..) => println!("Este não é um inteiro válido: {}", trimmed)
    };
	
//recebe a entrada do usuario
let mut numeroDeConsumidores = 10;

println!("Digite o numero de Consumidores:");
	let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => numeroDeConsumidores = i,
        Err(..) => println!("Este não é um inteiro válido: {}", trimmed)
    };
//recebe a entrada do usuario	
let mut tamanhoDoBuffer = 10;

println!("Digite o tamanho do Buffer:");
	let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => tamanhoDoBuffer = i,
        Err(..) => println!("Este não é um inteiro válido: {}", trimmed)
    };	
producerConsumerQuestion(numeroDeProdutores,numeroDeConsumidores,tamanhoDoBuffer);

}

fn producerConsumerQuestion(producerNumbers:i32, consumerNumbers:i32, bufferSize:i32){
let (tx, rx) = channel();

for i in 0..producerNumbers{
	
		let tx = tx.clone();
		//producer
		for j in 0..10*bufferSize {
			let tx = tx.clone();
			thread::spawn(move|| {
				let mut msg = "sou o produtor:".to_string();
				let id = i.to_string();
				let produz = "; Eu produzi:".to_string();
				let produzido = j.to_string();
				msg.push_str(&id); msg.push_str(&produz); msg.push_str(&produzido);

				tx.send(msg).unwrap();
			});
		}
		
	
}
	

for i in 0..consumerNumbers{

//consumer
	
		for _ in 0..10 {
			let j = rx.recv().unwrap();
			println!("Sou o consumidor:{} e consumi({})",i, j);
	    }
	
    
	
}

}