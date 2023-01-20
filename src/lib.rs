use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;

fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

macro_rules! make_i1d_implementation {
    ($($n:ident, $t:expr),+) => {
        $(
            #[pyfunction]
            pub fn $n(a: Vec<$t>, b: Vec<$t>) -> PyResult<(Vec<usize>, Vec<usize>)> {
                let mut a = a.to_owned();
                let indices = argsort(&a);
                a.sort();
                Ok(b.iter().enumerate().filter_map(|(i, &b_i)| {
                    let index = a.binary_search(&b_i);
                    if let Ok(index) = index {
                        Some((indices[index], i))
                    } else {
                        None
                    }
                }).unzip())
            }
        )+
    };
}

macro_rules! make_par_i1d_implementation {
    ($($n:ident, $t:expr),+) => {
        $(
            #[pyfunction]
            pub fn $n(a: Vec<$t>, b: Vec<$t>) -> PyResult<(Vec<usize>, Vec<usize>)> {
                let mut a = a.to_owned();
                let indices = argsort(&a);
                a.sort();
                Ok(b.par_iter().enumerate().filter_map(|(i, &b_i)| {
                    let index = a.binary_search(&b_i);
                    if let Ok(index) = index {
                        Some((indices[index], i))
                    } else {
                        None
                    }
                }).unzip())
            }
        )+
    };
}

// write the implementation for all the types
make_i1d_implementation!(
    i1d_i8, i8, i1d_i16, i16, i1d_i32, i32, i1d_i64, i64, i1d_u8, u8, i1d_u16, u16, i1d_u32, u32,
    i1d_u64, u64, i1d_isize, isize, i1d_usize, usize
);
make_par_i1d_implementation!(
    par_i1d_i8,
    i8,
    par_i1d_i16,
    i16,
    par_i1d_i32,
    i32,
    par_i1d_i64,
    i64,
    par_i1d_u8,
    u8,
    par_i1d_u16,
    u16,
    par_i1d_u32,
    u32,
    par_i1d_u64,
    u64,
    par_i1d_isize,
    isize,
    par_i1d_usize,
    usize
);

#[pymodule]
fn i1d(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(i1d_i8, m)?)?;
    m.add_function(wrap_pyfunction!(i1d_i16, m)?)?;
    m.add_function(wrap_pyfunction!(i1d_i32, m)?)?;
    m.add_function(wrap_pyfunction!(i1d_i64, m)?)?;
    m.add_function(wrap_pyfunction!(i1d_u8, m)?)?;
    m.add_function(wrap_pyfunction!(i1d_u16, m)?)?;
    m.add_function(wrap_pyfunction!(i1d_u32, m)?)?;
    m.add_function(wrap_pyfunction!(i1d_u64, m)?)?;
    m.add_function(wrap_pyfunction!(i1d_isize, m)?)?;
    m.add_function(wrap_pyfunction!(i1d_usize, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_i8, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_i16, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_i32, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_i64, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_u8, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_u16, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_u32, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_u64, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_isize, m)?)?;
    m.add_function(wrap_pyfunction!(par_i1d_usize, m)?)?;
    Ok(())
}
