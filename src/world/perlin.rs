use bevy::prelude::ops::sin;
use bevy::prelude::*;

pub fn noised(x: Vec3) -> Vec4 {
    let i = x.floor();
    let f = x - x.floor();

    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    let du = 30.0 * f * f * (f * (f - 2.0) + 1.0);

    // grad
    let ga = hash(Vec3::new(0., 0., 0.) + i);
    let gb = hash(Vec3::new(1., 0., 0.) + i);
    let gc = hash(Vec3::new(0., 1., 0.) + i);
    let gd = hash(Vec3::new(1., 1., 0.) + i);
    let ge = hash(Vec3::new(0., 0., 1.) + i);
    let gf = hash(Vec3::new(1., 0., 1.) + i);
    let gg = hash(Vec3::new(0., 1., 1.) + i);
    let gh = hash(Vec3::new(1., 1., 1.) + i);

    // Proj
    let va = ga.dot(f - Vec3::new(0., 0., 0.));
    let vb = gb.dot(f - Vec3::new(1., 0., 0.));
    let vc = gc.dot(f - Vec3::new(0., 1., 0.));
    let vd = gd.dot(f - Vec3::new(1., 1., 0.));
    let ve = ge.dot(f - Vec3::new(0., 0., 1.));
    let vf = gf.dot(f - Vec3::new(1., 0., 1.));
    let vg = gg.dot(f - Vec3::new(0., 1., 1.));
    let vh = gh.dot(f - Vec3::new(1., 1., 1.));

    let v: f32 = va
        + u.x * (vb - va)
        + u.y * (vc - va)
        + u.z * (ve - va)
        + u.x * u.y * (va - vb - vc + vd)
        + u.y * u.z * (va - vc - ve + vg)
        + u.z * u.x * (va - vb - ve + vf)
        + u.x * u.y * u.z * (-va + vb + vc - vd + ve - vf - vg + vh);

    let d: Vec3 = ga
        + u.x * (gb - ga)
        + u.y * (gc - ga)
        + u.z * (ge - ga)
        + u.x * u.y * (ga - gb - gc + gd)
        + u.y * u.z * (ga - gc - ge - gg)
        + u.z * u.x * (ga - gb - ge + gf)
        + u.x * u.y * u.z * (-ga + gb + gc - gd + ge - gf - gg + gh)
        + du * (vec3(vb - va, vc - va, ve - va)
            + u.yzx() * vec3(va - vb - vc + vd, va - vc - ve + vg, va - vb - ve + vf)
            + u.zxy() * vec3(va - vb - ve + vf, va - vb - vc + vd, va - vc - ve + vg)
            + u.yzx() * u.zxy() * (-va + vb + vc - vd + ve - vf - vg + vh));

    return vec4(v, d.x, d.y, d.z);
}

fn hash(p: Vec3) -> Vec3 {
    let p = Vec3::new(
        p.dot(vec3(127.1, 311.7, 74.7)),
        p.dot(vec3(269.5, 183.3, 246.1)),
        p.dot(vec3(113.5, 271.9, 124.6)),
    );

    -1.0 + 2.0 * (p.map(|x| sin(x)) * 43758.5453123).fract()
}
