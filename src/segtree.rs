use std::ops::Add;

pub struct SegTree<T> {
    arr: Vec<T>,
    s: usize,
}

impl<T> SegTree<T>
where T: Default + Clone + Copy + Add<Output = T>
{
    pub fn new(size: usize) -> Self {
        let mut s = 1;
        while s <= size {
            s *= 2;
        }

        SegTree { s, arr: vec![Default::default(); 2 * s] }
    }

    pub fn from(array: &Vec<T>) -> Self {
        let mut s = 1;
        while s <= array.len() {
            s *= 2;
        }

        let mut arr = vec![Default::default(); 2 * s];

        for (i, val) in array.iter().enumerate() {
            arr[i + s] = *val;
        }

        for i in (1..s).rev() {
            arr[i] = arr[2 * i] + arr[2 * i + 1];
        }

        SegTree { s, arr }
    }

    pub fn query(&self, l: usize, r: usize) -> T {
        let mut ans_l: T = Default::default();
        let mut ans_r: T = Default::default();

        let (mut l, mut r) = (l + self.s, r + self.s);
        while l < r {
            if (l & 1) == 1 {
                ans_l = ans_l + self.arr[l];
                l += 1;
            }
            if (r & 1) == 1 {
                r -= 1;
                ans_r = self.arr[r] + ans_r;
            }
        
            (l, r) = (l >> 1, r >> 1);
        }

        ans_l + ans_r
    }

    pub fn update(&mut self, pos: usize, val: &T) {
        let mut pos = pos + self.s;
        self.arr[pos] = *val;

        while {
            pos >>= 1;
            pos > 0
        } {
            self.arr[pos] = self.arr[2 * pos] + self.arr[2 * pos + 1];
        }

    }
}
