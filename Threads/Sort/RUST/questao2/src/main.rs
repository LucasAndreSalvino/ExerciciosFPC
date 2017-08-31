extern crate scoped_threadpool;
use scoped_threadpool::Pool;

fn main() {
    println!("Array desordenado:");
    let mut xs = vec![3,2,1,4,5,6,7];
	print!("[");
	for i in xs.iter() {
	    
        print!(" {}", i);
		
    }
	print!("]");
	println!("");
	let mut xss = paralell_sort(&xs, 7);
	println!("Array ordenado por 7 threads:");
	print!("[");
	for i in xss.iter() {
	    
        print!(" {}", i);
		
    }
	print!("]");
}

fn bubble_sort(mut vec_to_sort: &Vec<i32>, inicio:usize, fim:usize) -> Vec<i32> {
    let mut result = vec_to_sort.clone();
    for i in inicio..fim {
        for y in 0..vec_to_sort.len() as usize {
            if result[i] < result[y] {
                result.swap(i, y);
            }
        }
    }
    return result;
}

fn paralell_sort(mut vec_to_sort: &Vec<i32>, numeroDeThreads: i32) -> Vec<i32>{
	//inicia uma thread pool com o numero N passado no parametro 
	
	let mut pool = Pool::new(numeroDeThreads as u32);
	let mut vec = vec_to_sort.clone();
	
	
	let mut contador = 0 as usize;
	let mut index = 0;
	let mut numeroDePartes = vec.len()/(numeroDeThreads as usize);
	
	pool.scoped(|scoped| {
        
        for e in 0..numeroDeThreads{
            // substitui os itens da lista como mutavel 
            // (execute() is safe to call in nightly)
            unsafe {
               vec = bubble_sort(&vec, contador ,contador+numeroDePartes);

            }
			
			contador+=numeroDePartes;
			
        }
    });
	
	return vec;
	
}