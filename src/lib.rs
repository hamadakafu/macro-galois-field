//! Derive macro for Galois Field
//!
//! # Quick Start
//!
//! ```
//! use macro_galois_field::Field;
//!
//! #[derive(Field, Debug, Default, Copy, Clone)]
//! #[prime = 2]
//! struct Fp2(u64);
//!
//! let a = Fp2(3);
//! let b = Fp2(3);
//! assert_eq!(a + b, Fp2(0), "{} + {}", a.0, b.0);
//! assert_eq!(a - b, Fp2(0), "{} - {}", a.0, b.0);
//! assert_eq!(a - b, Fp2(2), "{} - {}", a.0, b.0);
//! assert_eq!(a * b, Fp2(1), "{} * {}", a.0, b.0);
//! assert_eq!(a * b, Fp2(3), "{} * {}", a.0, b.0);
//! assert_eq!(a / b, Fp2(1), "{} / {}", a.0, b.0);
//!
//! let a = Fp2(3);
//! let b = Fp2(100);
//! assert_eq!(a + b, Fp2(1), "{} + {}", a.0, b.0);
//! assert_eq!(a - b, Fp2(1), "{} - {}", a.0, b.0);
//! ```

#![recursion_limit="1024"]
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, DeriveInput,
};

// TODO: not only prime field, non-prime field
// TODO: not only struct(u64), but also struct(usize), struct {num: u32}
// TODO: check if prime is really a prime number
#[proc_macro_derive(Field, attributes(prime))]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let prime_attr = ast.attrs.iter().find(|atter| {
        atter.path.segments[0].ident.to_string() == "prime"
    }).expect("prime attr should exist.");
    let tt= prime_attr.tts.clone().into_iter().nth(0).expect(&format!("expect: #[prime = <num>].\nreal: {:?}", prime_attr.into_token_stream().to_string()));
    assert_eq!(tt.to_string(), "=");
    let prime_tt= prime_attr.tts.clone().into_iter().nth(1).expect(&format!("expect: #[prime = <num>].\nreal: {:?}", prime_attr.into_token_stream().to_string()));
    let prime: u64 = match prime_tt {
        proc_macro2::TokenTree::Literal(l) => {
            l.to_string().parse().unwrap()
        }
        _ => {
            panic!("{:?}", prime_tt)
        }
    };
    (quote!{
        impl #name {
            const prime: u64 = #prime;

            pub fn n(num: u64) -> Self {
                #name(num % #prime)
            }

            fn modinv(&self) -> Self {
                let mut x0: i64 = 1;
                let mut y0: i64 = 0;
                let mut x1: i64 = 0;
                let mut y1:i64 = 1;
                let mut a: i64 = self.0 as i64;
                let mut b: i64 = #prime as i64;
                while b != 0 {
                    let q = a / b;
                    let pre_b = b;
                    let pre_a = a;
                    a = pre_b;
                    b = pre_a % pre_b;

                    let pre_x0 = x0;
                    let pre_x1 = x1;
                    x0 = pre_x1;
                    x1 = pre_x0 - q * pre_x1;

                    let pre_y0 = y0;
                    let pre_y1 = y1;
                    y0 = pre_y1;
                    y1 = pre_y0 - q * pre_y1;
                }
                if a != 1 {
                    dbg!(a, b, x0, x1, y0, y1);
                    panic!("modular inverse does not exist for num: {}, moduler: {}", self.0, #prime);
                }
                if x0 < 0 {
                    let q = x0 / #prime as i64;
                    x0 -= (q - 1) * #prime as i64;
                }
                x0 = x0 % #prime as i64;
                #name(x0 as u64)
            }
        }

        impl std::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                #name((self.0 + rhs.0) % #prime)
            }
        }

        impl std::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 = (self.0 + rhs.0) % #prime;
            }
        }

        impl std::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                let mut n = self.0;
                while n < rhs.0 {
                    n += #prime
                }
                #name((n - rhs.0) % #prime)
            }
        }

        impl std::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                while self.0 < rhs.0 {
                    self.0 += #prime
                }
                self.0 = (self.0 - rhs.0) % #prime;
            }
        }

        impl std::ops::Mul for #name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                #name((self.0 * rhs.0) % #prime)
            }
        }

        impl std::ops::MulAssign for #name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 = (self.0 * rhs.0) % #prime;
            }
        }

        impl std::ops::Div for #name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                self * rhs.modinv()
            }
        }

        impl std::ops::DivAssign for #name {
            fn div_assign(&mut self, rhs: Self) {
                let a = #name(self.0) * rhs.modinv();
                self.0 = a.0;
            }
        }

        impl std::ops::Neg for #name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                let mut num = -(self.0 as i64);
                if num < 0 {
                    let q = num / #prime as i64;
                    num -= (q - 1) * #prime as i64;
                }

                num = num % #prime as i64;
                #name(num as u64)
            }
        }

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.0 % #prime == other.0 % #prime
            }
        }
        impl Eq for #name {}

    }).into()
}
