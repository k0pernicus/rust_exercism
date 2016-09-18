pub struct Triangle {
    sides: [u32; 3] 
}

impl Triangle {

    pub fn build(sides: [u32; 3]) -> Result<Triangle, ()> {
        if sides.iter().all(|&s| s > 0) &&
           sides[0] <= sides[1] + sides[2] &&
           sides[1] <= sides[0] + sides[2] &&
           sides[2] <= sides[0] + sides[1] {
               Ok(Triangle{sides: sides})
           } else {
               Err(()) 
           }
    }

    pub fn is_equilateral(&self) -> bool {
        if self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2] {
            true
        } else {
            false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        if (self.sides[0] == self.sides[1] && self.sides[0] != self.sides[2]) ||
           (self.sides[0] == self.sides[2] && self.sides[0] != self.sides[1]) ||
           (self.sides[1] == self.sides[2] && self.sides[0] != self.sides[2]) {
               true
           } else {
               false
           }
    }

    pub fn is_scalene(&self) -> bool {
        if self.sides[0] != self.sides[1] &&
           self.sides[1] != self.sides[2] &&
           self.sides[0] != self.sides[2] {
               true
        } else {
               false
        }
    }

}
