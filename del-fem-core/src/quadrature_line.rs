pub const DEGREE2IDX: [usize; 8] = [0, 1, 3, 6, 10, 15, 21, 28];

pub const IDX2POSWEIGHT_F32: [[f32; 2]; 28] = [
    // 1 point
    [0.0, 2.0],
    // 2 points [1-3]
    [-0.577350269189626, 1.0],
    [0.577350269189626, 1.0],
    // 3 points [3-6]
    [0.000000000000000, 0.888888888888889],
    [-0.774596669241483, 0.555555555555556],
    [0.774596669241483, 0.555555555555556],
    // 4 points [6-10]
    [-0.861136311594053, 0.347854845137454],
    [0.861136311594053, 0.347854845137454],
    [-0.339981043584856, 0.652145154862546],
    [0.339981043584856, 0.652145154862546],
    // 5 points [10-15]
    [0.0000000000000000, 0.5688888888888889],
    [-0.5384693101056831, 0.4786286704993665],
    [0.5384693101056831, 0.4786286704993665],
    [-0.9061798459386640, 0.2369268850561891],
    [0.9061798459386640, 0.2369268850561891],
    // 6 points [15-21]
    [0.6612093864662645, 0.3607615730481386],
    [-0.6612093864662645, 0.3607615730481386],
    [-0.2386191860831969, 0.4679139345726910],
    [0.2386191860831969, 0.4679139345726910],
    [-0.9324695142031521, 0.1713244923791704],
    [0.9324695142031521, 0.1713244923791704],
    // 7 points [21-28]
    [0.00000000000000000e+00, 4.17959183673469388e-01],
    [-4.05845151377397167e-01, 3.81830050505118945e-01],
    [4.05845151377397167e-01, 3.81830050505118945e-01],
    [-7.41531185599394440e-01, 2.79705391489276668e-01],
    [7.41531185599394440e-01, 2.79705391489276668e-01],
    [-9.49107912342758525e-01, 1.29484966168869693e-01],
    [9.49107912342758525e-01, 1.29484966168869693e-01],
];

pub const IDX2POSWEIGHT_F64: [[f64; 2]; 28] = [
    // 1 point
    [0.0, 2.0],
    // 2 points [1-3]
    [-0.577350269189626, 1.0],
    [0.577350269189626, 1.0],
    // 3 points [3-6]
    [0.000000000000000, 0.888888888888889],
    [-0.774596669241483, 0.555555555555556],
    [0.774596669241483, 0.555555555555556],
    // 4 points [6-10]
    [-0.861136311594053, 0.347854845137454],
    [0.861136311594053, 0.347854845137454],
    [-0.339981043584856, 0.652145154862546],
    [0.339981043584856, 0.652145154862546],
    // 5 points [10-15]
    [0.0000000000000000, 0.5688888888888889],
    [-0.5384693101056831, 0.4786286704993665],
    [0.5384693101056831, 0.4786286704993665],
    [-0.9061798459386640, 0.2369268850561891],
    [0.9061798459386640, 0.2369268850561891],
    // 6 points [15-21]
    [0.6612093864662645, 0.3607615730481386],
    [-0.6612093864662645, 0.3607615730481386],
    [-0.2386191860831969, 0.4679139345726910],
    [0.2386191860831969, 0.4679139345726910],
    [-0.9324695142031521, 0.1713244923791704],
    [0.9324695142031521, 0.1713244923791704],
    // 7 points [21-28]
    [0.00000000000000000e+00, 4.17959183673469388e-01],
    [-4.05845151377397167e-01, 3.81830050505118945e-01],
    [4.05845151377397167e-01, 3.81830050505118945e-01],
    [-7.41531185599394440e-01, 2.79705391489276668e-01],
    [7.41531185599394440e-01, 2.79705391489276668e-01],
    [-9.49107912342758525e-01, 1.29484966168869693e-01],
    [9.49107912342758525e-01, 1.29484966168869693e-01],
];

pub struct Quad<Real> {
    _marker: std::marker::PhantomData<fn() -> Real>,
}

pub trait Quadrature<Real> {
    fn hoge(i_gauss_degree: usize) -> &'static [[Real; 2]];
}

impl<Real> Quadrature<f64> for Quad<Real> {
    fn hoge(i_gauss_degree: usize) -> &'static [[f64; 2]] {
        let i0 = crate::quadrature_line::DEGREE2IDX[i_gauss_degree];
        let i1 = crate::quadrature_line::DEGREE2IDX[i_gauss_degree + 1];
        &crate::quadrature_line::IDX2POSWEIGHT_F64[i0..i1]
    }
}

impl<Real> Quadrature<f32> for Quad<Real> {
    fn hoge(i_gauss_degree: usize) -> &'static [[f32; 2]] {
        let i0 = crate::quadrature_line::DEGREE2IDX[i_gauss_degree];
        let i1 = crate::quadrature_line::DEGREE2IDX[i_gauss_degree + 1];
        &crate::quadrature_line::IDX2POSWEIGHT_F32[i0..i1]
    }
}
