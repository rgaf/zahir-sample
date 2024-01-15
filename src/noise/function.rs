use crate::{geometry::RealPoint, noise::{GRADIENTS_2D, GRADIENTS_3D}, random::HashFn, utils};

const PERLIN_BIAS_2D: f64 = 2.0f64 / std::f64::consts::SQRT_2;
const PERLIN_BIAS_3D: f64 = 1.1547005383792517;

pub fn perlin_1d<T: HashFn>(hash_fn: &T, point: RealPoint<1>) -> f64 {
    let px = point[0];

    let ax0 = px.floor();
    let ax1 = ax0 + 1.0;

    let v0 = vertex_1d(hash_fn, ax0);
    let v1 = vertex_1d(hash_fn, ax1);

    let vx = utils::cerp(px - ax0, v0, v1);

    utils::smoothstep(vx)
}

fn vertex_1d<T: HashFn>(hash_fn: &T, x: f64) -> f64 {
    let hash = hash_fn.hash_1u64(x.to_bits());

    utils::f64_from_mantissa(hash, 0.0, 1.0)
}

pub fn perlin_2d<T: HashFn>(hash_fn: &T, point: RealPoint<2>) -> f64 {
    let px = point[0];
    let py = point[1];

    let ax0 = px.floor();
    let ay0 = py.floor();

    let ax1 = ax0 + 1.0;
    let ay1 = ay0 + 1.0;

    let nx0 = px - ax0;
    let ny0 = py - ay0;

    let nx1 = nx0 - 1.0;
    let ny1 = ny0 - 1.0;

    let v00 = vertex_2d(hash_fn, ax0, ay0, nx0, ny0);
    let v10 = vertex_2d(hash_fn, ax1, ay0, nx1, ny0);
    let v01 = vertex_2d(hash_fn, ax0, ay1, nx0, ny1);
    let v11 = vertex_2d(hash_fn, ax1, ay1, nx1, ny1);

    let sx = utils::smoothstep(nx0);
    let sy = utils::smoothstep(ny0);

    let vx0 = utils::lerp(sx, v00, v10);
    let vx1 = utils::lerp(sx, v01, v11);

    let vxy = utils::lerp(sy, vx0, vx1);

    utils::smoothstep(utils::neg_unit_to_unit(vxy * PERLIN_BIAS_2D))
}

fn vertex_2d<T: HashFn>(hash_fn: &T, ax: f64, ay: f64, nx: f64, ny: f64) -> f64 {
    let hash = hash_fn.hash_2u64(ax.to_bits(), ay.to_bits()) as usize;
    let (gx,  gy) = GRADIENTS_2D[hash & 31];

    gx * nx + gy * ny
}

pub fn perlin_3d<T: HashFn>(hash_fn: &T, point: RealPoint<3>) -> f64 {
    let px = point[0];
    let py = point[1];
    let pz = point[2];

    let ax0 = px.floor();
    let ay0 = py.floor();
    let az0 = pz.floor();

    let ax1 = ax0 + 1.0;
    let ay1 = ay0 + 1.0;
    let az1 = az0 + 1.0;

    let nx0 = px - ax0;
    let ny0 = py - ay0;
    let nz0 = pz - az0;

    let nx1 = nx0 - 1.0;
    let ny1 = ny0 - 1.0;
    let nz1 = nz0 - 1.0;

    let v000 = vertex_3d(hash_fn, ax0, ay0, az0, nx0, ny0, nz0);
    let v100 = vertex_3d(hash_fn, ax1, ay0, az0, nx1, ny0, nz0);
    let v010 = vertex_3d(hash_fn, ax0, ay1, az0, nx0, ny1, nz0);
    let v110 = vertex_3d(hash_fn, ax1, ay1, az0, nx1, ny1, nz0);
    let v001 = vertex_3d(hash_fn, ax0, ay0, az1, nx0, ny0, nz1);
    let v101 = vertex_3d(hash_fn, ax1, ay0, az1, nx1, ny0, nz1);
    let v011 = vertex_3d(hash_fn, ax0, ay1, az1, nx0, ny1, nz1);
    let v111 = vertex_3d(hash_fn, ax1, ay1, az1, nx1, ny1, nz1);

    let sx = utils::smoothstep(nx0);
    let sy = utils::smoothstep(ny0);
    let sz = utils::smoothstep(nz0);

    let vx00 = utils::lerp(sx, v000, v100);
    let vx10 = utils::lerp(sx, v010, v110);
    let vx01 = utils::lerp(sx, v001, v101);
    let vx11 = utils::lerp(sx, v011, v111);

    let vxy0 = utils::lerp(sy, vx00, vx10);
    let vxy1 = utils::lerp(sy, vx01, vx11);

    let vxyz = utils::lerp(sz, vxy0, vxy1);

    utils::smoothstep(utils::neg_unit_to_unit(vxyz * PERLIN_BIAS_3D))
}

fn vertex_3d<T: HashFn>(hash_fn: &T, ax: f64, ay: f64, az: f64, nx: f64, ny: f64, nz: f64) -> f64 {
    let hash = hash_fn.hash_3u64(ax.to_bits(), ay.to_bits(), az.to_bits()) as usize;
    let (gx,  gy, gz) = GRADIENTS_3D[hash.rotate_left(4) & 15];

    gx * nx + gy * ny + gz * nz
}
