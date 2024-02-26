use super::num::{
    CommonField,
    Complex,
    PI,
};

struct BitRevIterator 
{
    a: usize,
    n: usize,
}
impl BitRevIterator
{
    fn new(
        n: usize
    ) -> Self {
        assert!(n.is_power_of_two());
        Self { a: 2 * n - 1, n}
    }
}

pub trait FFT: Sized + Copy
{
    type F: Sized
        + Copy
        + From<Self>
        + Neg
        + Add<Output = Self::F>
        + Div<Output = Self::F>
        + Mul<Output = Self::F>
        + Sub<Output = Self::F>;

    const ZERO: Self;

    fn get_roots(
        n: usize,
        inverse: bool
    ) -> Vec<Self::F>;

    fn get_factor(
        n: usize,
        inverse: bool
    ) -> Self::F;

    fn extract(
        f: Self::F
    ) -> Self;
}
impl FFT for f64 
{
    type F = Complex;
    
    const ZERO: f64 = 0.0;

    fn get_roots(
        n: usize,
        inverse: bool
    ) -> Vec<Self::F> {
        let step = if inverse { -2.0 } else { 2.0 } * PI / n as f64;
        (0..n / 2)
            .map( |i| Complex::from_polar(1.0, step * i as f64))
            .collect()
    }

    fn get_factor(
        n: usize,
        inverse: bool
    ) -> Self::F {
        Self::F::from( if inverse { n as Self } else { 1 }).recip()
    }
}

pub fn fft<T: FFT>(
    v: &[T::F],
    inverse: bool
) -> Vec<T::F> {
    let n = v.len();
    assert!(n.is_power_of_two());

    let factor = T::get_factor(n, inverse);
    let roots_of_unity = T::get_roots(n, inverse);
    let mut dft = BitRevIterator::new(n)
                    .map( |i| v[i] * factor)
                    .collect::<Vec<_>>();

    for m in (0..).map( |s| 1 << s).take_while( |&m| m < n) {
        for k in (0..n).step_by(2 * m) {
            for j in 0..m {
                let u = dft[k + j];
                let t = dft[k + j + m] * roots_of_unity[n / 2 / m * j];
                dft[k + j] = u + t;
                dft[k + j + m] = u - t;
            }
        }
    }
    dft
}

pub fn dft_from_reals<T: FFT>(
    v: &[T],
    desired_len: usize,
) -> Vec<T::F> {
    assert!(v.len() <= desired_len);

    /*
        · iter(): 迭代实数切片中的元素
        · cloned(): 复制每个元素
        · chain(): 将std::iter::repeat(T::ZERO)生成的无限迭代器连接在实数切片的末尾。
            这样做是为了将输入的实数切片扩展到所需的长度，并在末尾填充零值。
        · take(): 限制切片长度为大于等于所需长度的下一个2的幂
        · map(): 将每个元素转换为对应的复数类型
        · collect(): 将转换后的元素收集到一个Vec中
    */
    let complex_v = v
                    .iter()
                    .cloned()
                    .chain(std::iter::repeat(T::ZERO))
                    .take(desired_len.next_power_of_two())
                    .map(T::F::from)
                    .collect::<Vec<_>>();
    fft::<T>(&complex_v, false)
}

pub fn idft_to_reals<T: FFT>(
    dft_v: &[T::F],
    desired_len: usize
) -> Vec<T> {
    assert!(dft_v.len() >= desired_len);

    let complex_v = fft::<T>(dft_v, true);
    complex_v
            .into_iter()
            .take(desired_len)
            .map(T::extract)
            .collect()
}