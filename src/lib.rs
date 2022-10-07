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
