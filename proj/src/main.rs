use std::cmp::min;
use std::time::Instant;

fn main() {
    let matrix_size = 3000;
    let block_size = 8;
    on_mult(matrix_size);
    line_mult(matrix_size);
    block_mult(matrix_size, block_size);
}

fn on_mult(matrix_size: usize){

    let pha = vec![1.0; matrix_size*matrix_size];
    let mut phb = vec![0.0; matrix_size*matrix_size];
    let mut phc = vec![0.0; matrix_size*matrix_size];

    for i in 0..matrix_size {
        for j in 0..matrix_size {
            phb[i*matrix_size + j] = i as f64 + 1.0;
        }
    }

    let start = Instant::now();

    for i in 0..matrix_size {
        for j in 0..matrix_size {
            let mut temp = 0.0;
            for k in 0..matrix_size{
                temp += pha[i*matrix_size + k] * phb[k*matrix_size+j];
            }
            phc[i*matrix_size+j] = temp;
        }
    }

    let elapsed = Instant::now() - start;

    // result
    for i in 0..min(10,matrix_size) {
        print!("{} ", phc[i]);
    }
    println!();
    println!("Elapsed time: {} seconds and {} milliseconds", elapsed.as_secs(), elapsed.as_millis());
    println!();
}

fn line_mult(matrix_size: usize){
    
    let pha = vec![1.0; matrix_size*matrix_size];
    let mut phb = vec![0.0; matrix_size*matrix_size];
    let mut phc = vec![0.0; matrix_size*matrix_size];

    for i in 0..matrix_size {
        for j in 0..matrix_size {
            phb[i*matrix_size + j] = i as f64 + 1.0;
        }
    }
    let start = Instant::now();

    for i in 0..matrix_size {
        for k in 0..matrix_size {
            for j in 0..matrix_size {
                phc[i*matrix_size + j] += pha[i*matrix_size + k] * phb[k*matrix_size + j];
            }
        }
    }

    let elapsed = Instant::now() - start;
    // result
    for i in 0..min(10,matrix_size) {
        print!("{} ", phc[i]);
    }
    println!();
    println!("Elapsed time: {} seconds and {} milliseconds", elapsed.as_secs(), elapsed.as_millis());
    println!();

}

fn block_mult(matrix_size: usize, block_size: usize) {

    if matrix_size % block_size != 0{
        println!("ERROR: Matrix is undivisible by the block size");
        return;
    }

    let pha = vec![1.0; matrix_size*matrix_size];
    let mut phb = vec![0.0; matrix_size*matrix_size];
    let mut phc = vec![0.0; matrix_size*matrix_size];

    for i in 0..matrix_size{
        for j in 0..matrix_size{
            phb[i*matrix_size + j] = i as f64 + 1.0;
        }
    }
    
    let start = Instant::now();
    // divide matrixes into square matrixes until we get all blocks of 2x2 matrixes
    // then do internal mult, then

    // MOVE BLOCKS AROUND
    for block_i in (0..matrix_size).step_by(block_size) {
        for block_k in (0..matrix_size).step_by(block_size) {
            for block_j in (0..matrix_size).step_by(block_size) {

                // CALCULATE INSIDE EACH BLOCK MATRIX
                for i in block_i..(block_i + block_size) {
                    for k in block_k..(block_k + block_size) {
                        for j in block_j..(block_j + block_size) {
                            phc[i*matrix_size + j] += pha[i*matrix_size + k] * phb[k*matrix_size + j];
                        }
                    }
                }

            }
        }
    }

    let elapsed = Instant::now() - start;
    // result
    for i in 0..min(10,matrix_size) {
        print!("{} ", phc[i]);
    }
    println!();
    println!("Elapsed time: {} seconds and {} milliseconds", elapsed.as_secs(), elapsed.as_millis());
    println!();

}
