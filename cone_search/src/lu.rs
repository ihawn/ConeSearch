use ndarray::{Array1, Array2};

pub fn lu_solve(a: Array2<f64>, b: Array1<f64>, n: usize) -> Array1<f64>
{
    let mut l: Array2<f64> = Array2::eye(n);
    let mut u: Array2<f64> = Array2::zeros((n,n));

    //LU
    for i in 0..n
    {
        for k in i..n
        {
            let mut sum = 0.0;
            for j in 0..i
            {
                sum += l[(i,j)]*u[(j,k)];
            }
            u[(i,k)] = a[(i,k)] - sum;
        }
        for k in i+1..n
        {
            let mut sum = 0.0;
            for j in 0..i
            {
                sum += l[(k,j)]*u[(j,i)];
            }
            l[(k,i)] = (a[(k,i)] - sum) / u[(i,i)];
        }
    }
 
    //Lz = b
    let mut z: Array1<f64> = Array1::zeros(n);
    for i in 0..n
    {
        z[i] = b[i];
        for j in 0..i
        {
            z[i] -= l[(i,j)]*z[j]
        }
        z[i] /= l[(i,i)];
    }

    //Ux = z
    let mut x: Array1<f64> = Array1::zeros(n);
    for i in (0..n).rev()
    {
        x[i] = z[i];
        for j in i+1..n
        {
            x[i] -= u[(i,j)]*x[j]
        }
        x[i] /= u[(i,i)];
    }

    x
}