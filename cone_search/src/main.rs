mod structs;
mod lu;
mod intersections;
mod cone_search;
mod pyramid_handler;
use std::time::Instant;
use std::io;
use csv;
use std::error::Error;

fn main()
{
    let before = Instant::now();
    cone_search::solve((-8.0, 7.0), (-9.0, 10.0), 125.0, 150, 100000);
    println!("Elapsed time: {:.2?}", before.elapsed());
    io::stdin().read_line(&mut String::new()).unwrap();

    //Test runtime vs L
    // let mut L: Vec<f64> = vec!();
    // let mut T: Vec<f64> = vec!();
    // for i in 1..100
    // {
    //     let n = i as f64 * 0.02; 
    //     let Lv = f64::sqrt(f64::powf(n*2.0*2.0, 2.0)*2.0);
    //     let before = Instant::now();
    //     cone_search::solve((-2.0, 2.0), (-2.0, 2.0), Lv, 100, 100000, n);
    //     L.push(Lv);
    //     T.push(before.elapsed().as_secs_f64());
    // }
    // if let Err(e) = write_to_csv("C:/Users/Isaac/Documents/Optimization/NonConvex/NonConvexOptimization/NonConvexOptimiztion/BasinHopping/csv/ConeSearchVariableLRuntimes.csv", L, T)
    // {
    //     eprint!("{}", e);
    // }
}

fn write_to_csv(path: &str, l: Vec<f64>, t: Vec<f64>) -> Result<(), Box<dyn Error>>
{
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record(&["Runtime", "L"])?;

    for i in 0..l.len()
    {
        writer.write_record(&[t[i].to_string(), l[i].to_string()])?;
    }
    writer.flush()?;

    Ok(())
}

