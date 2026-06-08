pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let (a, b, c) = (sides[0], sides[1], sides[2]);
        if a > 0 && b > 0 && c > 0 && a + b >= c && a + c >= b && b + c >= a {
            Some(Triangle { a, b, c })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.a != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.a == self.c
    }
}
