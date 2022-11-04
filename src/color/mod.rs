#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// RGBA color value representation with four unsigned 8-bit ints.
pub struct Color {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8,
}

impl Color {
    /// Initialize a new color.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Adds one color to self and returns a new instance of the result.
    ///
    /// **Note:** This uses `wrapping_add` to avoid overflow errors.
    pub fn add(&self, rhs: &Color) -> Self {
        Self {
            r: self.r.wrapping_add(rhs.r),
            g: self.g.wrapping_add(rhs.g),
            b: self.b.wrapping_add(rhs.b),
            a: self.a.wrapping_add(rhs.a),
        }
    }

    /// Subtracts one color from self and returns a new instance of the result.
    ///
    /// **Note:** This uses `wrapping_sub` to avoid overflow errors.
    pub fn sub(&self, rhs: &Color) -> Self {
        Self {
            r: self.r.wrapping_sub(rhs.r),
            g: self.g.wrapping_sub(rhs.g),
            b: self.b.wrapping_sub(rhs.b),
            a: self.a.wrapping_sub(rhs.a),
        }
    }

    /// Calculates a range of colors between self and target returning the result as a vector of colors
    pub fn gradient_of(&self, target: &Color, steps: u32) -> Vec<Color> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_basic() {
        let lhs = Color::new(0x00, 0x00, 0x00, 0xff);
        let rhs = Color::new(0xff, 0xff, 0x00, 0x00);
        assert_eq!(Color::new(0xff, 0xff, 0x00, 0xff), lhs.add(&rhs));
    }

    #[test]
    fn add_wrapping() {
        let lhs = Color::new(128, 128, 128, 255);
        let rhs = Color::new(255, 255, 128, 255);
        assert_eq!(Color::new(127, 127, 0, 254), lhs.add(&rhs));
    }

    #[test]
    fn sub_basic() {
        let lhs = Color::new(0xff, 0xff, 0x00, 0xff);
        let rhs = Color::new(0x7f, 0x7f, 0x00, 0xff);
        assert_eq!(Color::new(0x80, 0x80, 0x00, 0x00), lhs.sub(&rhs));
    }

    #[test]
    fn sub_wrapping() {
        let lhs = Color::new(0x00, 0x00, 0x7f, 0xff);
        let rhs = Color::new(0x80, 0x80, 0xff, 0xff);
        assert_eq!(Color::new(0x80, 0x80, 0x80, 0x00), lhs.sub(&rhs));
    }

    #[test]
    #[ignore = "unimplemented"]
    fn gradient_linear() {
        let lhs = Color::new(0x00, 0x00, 0x00, 0x00);
        let rhs = Color::new(0xff, 0xff, 0xff, 0xff);
        let expected = vec![Color::new(0, 0, 0, 0)]; // pending
        let actual = lhs.gradient_of(&rhs, 5);

        println!("base: {:?}\ntarget: {:?}", lhs, rhs);

        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore = "unimplemented"]
    fn gradient_nonlinear() {
        let lhs = Color::new(0x00, 0xff, 0xff, 0xff);
        let rhs = Color::new(0xff, 0x00, 0x00, 0x00);
        let expected = vec![Color::new(0, 0, 0, 0)]; // pending
        let actual = lhs.gradient_of(&rhs, 4);

        println!("base: {:?}\ntarget: {:?}", lhs, rhs);

        assert_eq!(expected, actual);
    }
}
