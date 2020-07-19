use macro_galois_field::Field;

#[derive(Field)]
#[prime = 2]
struct Fp(u64);
fn main() {
    let _f = Fp(3);
}
