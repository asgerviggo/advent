use std::{fmt::Debug, ops::BitXor, str::FromStr};

pub fn parse<T: FromStr>(val: &str) -> T {
    match val.parse::<T>() {
        Ok(out) => out,
        Err(_) => panic!("Cannot parse: {}", { val }),
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct BoolVec(Vec<bool>);

impl BoolVec {
    fn from(vec: Vec<bool>) -> BoolVec {
        BoolVec(vec)
    }
}

// From rust cookbook
impl BitXor for BoolVec {
    type Output = Self;

    fn bitxor(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        Self(
            lhs.iter()
                .zip(rhs.iter())
                .map(|(x, y)| *x ^ *y)
                .collect(),
        )
    }
}

impl FromIterator<bool> for BoolVec {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut out = Vec::new();
        for entry in iter {
            out.push(entry);
        }
        BoolVec::from(out)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Point3D(isize, isize, isize);

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Point2D(isize, isize);

#[allow(dead_code)]
fn recursplit(string: String, mid: usize) -> Vec<String> {
    match string.split_at_checked(mid) {
        Some((left, right)) => {
            [vec![left.to_string()], recursplit(right.to_string(), mid)]
                .concat()
        }
        None => vec![string],
    }
}
