use rand::prelude::*;
use std::time::Instant;

#[derive(Debug)]
struct Matrix {
    data : Vec<f32>,
    rows : usize,
    cols : usize
}

fn mat_mul(a: &Matrix, b:  &Matrix)-> Matrix {
    let mut out : Vec<f32> = vec![0.0;a.rows * b.cols];
    for i in 0..a.rows{
        for j in 0..b.cols{
            for k in 0..a.cols{
                out[i*b.cols + j] += a.data[i*a.cols+k] * b.data[k*b.cols + j]; 
                }
            }
        }
    let out_m = Matrix {data : out, rows : a.rows, cols : b.cols};
    return out_m;
}


fn main() {
    let n:usize = 500;
    let mut rng = rand::thread_rng();
    let rana : Vec<f32> = (0..n*n).map(|_| 2.0*rng.gen::<f32>() - 1.0).collect();
    let ranb : Vec<f32> = (0..n*n).map(|_| 2.0*rng.gen::<f32>() - 1.0).collect();
    let testa = Matrix{data : rana, rows : n, cols: n};
    let testb = Matrix{data :ranb, rows: n , cols: n};
    let now = Instant::now();
    let _testc = mat_mul(&testa,&testb);
    let elapsed = now.elapsed();
    println!("time {:.2?}",elapsed);
    //println!("{:?}",testc); 
}

