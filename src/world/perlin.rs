use bevy::math::{ops::floor, Vec3};

const PERMUTATION: [u32; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

const fn concat_const(p: [u32; 256]) -> [u32; 512] {
    let mut out: [u32; 512] = [0; 512];
    let mut i = 0;
    loop {
        if i >= 256 {
            break;
        }
        out[i] = p[i];
        out[256 + i] = p[i];
        i += 1;
    }

    out
}

const P: [u32; 512] = concat_const(PERMUTATION);

pub fn noise(mut x: f32, mut y: f32, mut z: f32) -> f32 {
    let lx = floor(x) as u32 & 255;
    let ly = floor(y) as u32 & 255;
    let lz = floor(z) as u32 & 255;

    x -= floor(x);
    y -= floor(y);
    z -= floor(z);

    let u: f32 = fade(x);
    let v: f32 = fade(y);
    let w: f32 = fade(z);

    let a = P[lx as usize] + ly;
    let aa = P[a as usize] + lz;
    let ab = P[a as usize + 1] + ly;
    let b = P[lx as usize + 1] + ly;
    let ba = P[b as usize] + lz;
    let bb = P[b as usize + 1] + lz;

    return lerp(
        w,
        lerp(
            v,
            lerp(
                u,
                grad(P[aa as usize], x, y, z),
                grad(P[ba as usize], x - 1., y, z),
            ),
            lerp(
                u,
                grad(P[ab as usize], x, y - 1., z),
                grad(P[bb as usize], x - 1., y - 1., z),
            ),
        ),
        lerp(
            v,
            lerp(
                u,
                grad(P[aa as usize + 1], x, y, z - 1.),
                grad(P[ba as usize + 1], x - 1., y, z - 1.),
            ),
            lerp(
                u,
                grad(P[ab as usize + 1], x, y - 1., z - 1.),
                grad(P[bb as usize + 1], x - 1., y - 1., z - 1.),
            ),
        ),
    );
    // return p.x*p.z;
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6. - 15.) + 10.)
    // t^5 * 6 - 15t + 10t^2
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

fn grad(hash: u32, x: f32, y: f32, z: f32) -> f32 {
    let h = hash & 15; // Convert L0 4 bits of hash code
    let u = if h < 8 { x } else { y }; // INTO 12 GRADIENT DIRECTIONS
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };

    (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
}
