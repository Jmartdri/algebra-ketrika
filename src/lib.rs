mod object_op {
    use std::ops::Add;

    impl Add<Kanakana> for Giroka {
        type Output = Option<Sary>;

        fn add(self, rhs: Kanakana) -> Self::Output {
            if self.0 == LH::L && rhs.0 == LH::L {
                None
            } else if self.0 == LH::V && rhs.0 == LH::V {
                None
            } else {
                Some(Sary { isany: 10 })
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum LH {
        L,
        V,
    }

    #[derive(Debug, PartialEq)]
    struct Giroka(LH);

    #[derive(Debug, PartialEq)]
    struct Kanakana(LH);

    #[derive(Debug, PartialEq)]
    struct Sary {
        isany: u8,
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_couple() {
            let sary = Giroka(LH::L) + Kanakana(LH::V);
            assert_eq!(
                sary,
                Some(Sary { isany: 10 }),
                "Giroka lahy + Kanakana vavy"
            );

            let sary = Giroka(LH::L) + Kanakana(LH::L);
            assert_eq!(sary, None, "Giroka lahy + Kanakana lahy");

            let sary = Giroka(LH::V) + Kanakana(LH::V);
            assert_eq!(sary, None, "Giroka vavy + Kanakana vavy");

            let sary = Giroka(LH::V) + Kanakana(LH::L);
            assert_eq!(
                sary,
                Some(Sary { isany: 10 }),
                "Giroka vavy + Kanakana lahy"
            );
        }
    }
}

mod vecteur {

    use std::ops::Add;

    #[allow(dead_code)]
    fn round(decimal: f32, precision: u8) -> f32 {
        let p = 10u8.pow(precision as u32) as f32;
        (decimal * p).round() / p
    }

    #[derive(Debug, PartialEq)]
    struct Vecteur<X, Y> {
        x: X,
        y: Y,
    }

    impl<X, Y> Add for Vecteur<X, Y>
    where
        X: Add<Output = X>,
        Y: Add<Output = Y>,
    {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            return Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            };
        }
    }

    #[test]
    fn test_add_vecteur() {
        fn round2(decimal: f32) -> f32 {
            round(decimal, 2)
        }

        let ab = Vecteur { x: 1.5, y: 5 };
        let bc = Vecteur { x: -2.2, y: 3 };
        let ac = ab + bc;
        let expected = Vecteur { x: -0.7, y: 8 };
        assert_eq!(round2(ac.x as f32), round2(expected.x as f32));
        assert_eq!(round2(ac.y as f32), round2(expected.y as f32));
    }
}
