extern crate scoped_threadpool;
use scoped_threadpool::Pool;
use std::io;
fn main() {
    let mut nivelDeParalelismo = 10;
	println!("Digite o nivel de paralelismo como inteiro");
	let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => nivelDeParalelismo = i,
        Err(..) => println!("Este não é um inteiro válido: {}", trimmed)
    };
	println!("Digite as notas separadas por espaço");
	let mut notasLatinas = String::new();
    io::stdin()
        .read_line(&mut notasLatinas)
        .expect("failed to read from stdin");
    println!("- - - - - - - - - - - - - - Result - - - - - - - - - - - - - ");
	println!("Numero de Threads trabalhando:{}", nivelDeParalelismo);
	
	// cria uma thread pool com tamanho passado pelo usuario.
	
    let mut pool = Pool::new(nivelDeParalelismo);
	let mut vec: Vec<&str> = notasLatinas.trim().split(" ").collect();
    //let  mut vec = vec!["Do".to_string(), "Re".to_string(), "Mi".to_string(), "Fa".to_string(), "Sol".to_string()];

    // Cria uma thread pool com scoped para poder iterar atraves do loop for.
    
    pool.scoped(|scoped| {
        // Create references to each element in the vector ...
        for e in &mut vec {
            // substitui os itens da lista como mutavel 
            // (execute() is safe to call in nightly)
            unsafe {
                scoped.execute(move || {
                   if str::eq(e,"Do") {
						*e=("C");
					} else if str::eq(e,"Re") {
					   *e=("D");
					} else if str::eq(e,"Mi") {
					   *e=("E");
					} else if str::eq(e,"Fa") {
					   *e=("F");
					} else if str::eq(e,"Sol") {
					   *e=("G");
					} else if str::eq(e,"La") {
					   *e=("A");
					} else if str::eq(e,"Si") {
					   *e=("B");
					}
                });
            }
        }
    });
	println!("Notas Resultantes:");
	for i in vec.iter() {
        print!(" {}", i);
    }
   
}

