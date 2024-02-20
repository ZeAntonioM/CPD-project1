use std::cmp::min;

fn main() {
    let m_ar = 5;
    let m_br = 5;
    on_mult(m_ar, m_br);
}

fn on_mult(m_ar: usize, m_br: usize) {
    let pha = vec![1.0; m_ar*m_ar];
    let mut phb = vec![0.0; m_br*m_br];
    let mut phc = vec![0.0; m_ar*m_br];

    for i in 0..m_br {
        for j in 0..m_br {
            phb[i*m_br + j] = i as f64 + 1.0;
        }
    }

    for i in 0..m_ar {
        for j in 0..m_br {
            let mut temp = 0.0;
            for k in 0..m_ar{
                temp += pha[i*m_ar + k] * phb[k*m_br+j];
            }
            phc[i*m_ar+j] = temp;
        }
    }

    // result
    for i in 0..min(10,m_br) {
        print!("{} ", phc[i]);
    }
    println!();
}
