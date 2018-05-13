use na::Vector3 as Vec3;

use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq)]
pub struct Surface {
    pub pixels: Vec<u8>,
    pub w: u32,
    pub h: u32
}

#[derive(Hash, PartialEq, Eq)]
pub struct Division<'a> {
    pub x0: u32,
    pub y0: u32,
    pub w: u32,
    pub h: u32,
    pub surf: &'a Surface,
}

impl Surface {
    pub fn new(w: u32, h: u32) -> Surface {
        let depth = 3;
        let cap = (h * w * depth) as usize;
        let mut p = Vec::with_capacity(cap);
        unsafe {
            p.set_len(cap);
        }
        let surf = Surface { w: w, h: h, pixels: p };
        surf
    }

    /// dw -- division quadrant width
    /// dh -- division quadrant height
    pub fn divide(&self, dw: u32, dh: u32) -> HashSet<Division> {
        let mut w_overflow = false;
        let mut h_overflow = false;
        let w_num = self.w / dw +
            if self.w % dw != 0 { w_overflow = true; 1} else { 0 };
        let h_num = self.h / dh +
            if self.h % dh != 0 { h_overflow = true; 1 } else { 0 };

        let mut divisions = HashSet::new();
        for i in 0..w_num {
            for j in 0..h_num {
                let w = if w_overflow && i == w_num - 1 {
                    self.w - dw * (w_num - 1)
                } else { dw };

                let h = if h_overflow && j == h_num - 1 {
                    self.h - dh * (h_num - 1)
                } else { dh };

                divisions.insert(Division { x0: i * dw, y0: j * dh, w: w, h: h, surf: self });
            }
        }
        divisions
    }
}

impl<'a> Division<'a> {
    pub fn set_color(&self, x: u32, y: u32, color: Vec3<f64>) {
        let offset = ((y * self.surf.w + x) * 3) as usize;
        let pixels = &self.surf.pixels as *const Vec<u8> as *mut Vec<u8>;
        unsafe {
            (*pixels)[offset] = (color.x * 255.) as u8;
            (*pixels)[offset + 1] = (color.y * 255.) as u8;
            (*pixels)[offset + 2] = (color.z * 255.) as u8;
        }
    }
}
