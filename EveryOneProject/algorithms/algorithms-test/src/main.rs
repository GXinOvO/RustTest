pub mod math;

pub use std::f64::consts::PI;
use math::num::{
    Rational,
    Complex,
    Modulo,
    CommonField,
    Matrix,
};

fn main() {
    /* 
    --TODO: num.rs
        let three = Rational::from(3);
        let six = Rational::from(6);
        let three_and_half = three + three / six;
        let half = three_and_half / six;
        let half_sub_six = half - six;

        println!("three_and_six: {:#?}", three_and_half);
        println!("half: {:#?}", half);
        println!("half_sub_six: {:#?}", half_sub_six);
        println!("{:#?}", half_sub_six > Rational::from(3));
        println!("{:#?}", half_sub_six > Rational::from(-5));

        let minus_three_and_half = six - three_and_half + three / (-three / six);
        println!("minus_three_and_half: {:#?}", minus_three_and_half);
        let zero = three_and_half + minus_three_and_half;
        println!("zero: {:#?}", zero);

        let four = Complex::new(4.0, 0.0);
        let two_i = Complex::new(0.0, 2.0);

        let four_div_two = four / two_i;
        println!("four_div_two: {:#?}", four_div_two);
        let two_mul_two = two_i * -two_i;
        println!("two_mul_two: {:#?}", two_mul_two);
        let zero = two_i - two_i;
        println!("zero: {:#?}", zero);
        println!("four_sq: {:#?}", four.abs_square());
        println!("two_sq: {:#?}", two_i.abs_square());
        let four_neg_argu = (-four).argument();
        println!("four_neg_argu: {:#?}", four_neg_argu);
        let four_argu = (four).argument();
        println!("four_argu: {:#?}", four_argu);

        let base = CommonField::from(1234);
        let zero = base - base;
        println!("zero: {:#?}", zero);
        let one = base.recip() * base;
        println!("one: {:#?}", one);
        let one_add_one = one + one ;
        println!("one_add_one: {:#?}", one_add_one);
        let mixture = one / base * (base * base) - base / one;
        println!("mixture: {:#?}", mixture);

        let recips = CommonField::vec_of_recips(20);
        println!("recips_len: {:#?}", recips.len());

        let zero = Matrix::zero(2, 2);
        println!("zero: {:#?}", zero);
        let one = Matrix::one(2);
        println!("one: {:#?}", one);
        let rotate_90 = Matrix {
            cols: 2,
            inner: Box::new([0.0, -1.0, 1.0, 0.0]),
        };
        println!("rotate_90: {:#?}", rotate_90);

        let x_vec = Matrix::vector(&[1.0, 0.0], false);
        let y_vec = Matrix::vector(&[0.0, 1.0], false);
        let x_dot_x = &x_vec.transpose() * &x_vec;
        let x_dot_y = &x_vec.transpose() * &y_vec;
        println!("x_dot_x: {:#?}", x_dot_x);
        println!("x_dot_y: {:#?}", x_dot_y);
    */
    
    let v = vec![7.0, 1.0, 1.0];
    let dft_v = dft_from_reals(&v, v.len());
    println!("dft_v: {:#?}", dft_v);

}

