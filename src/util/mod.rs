//! Various utilities

pub mod units;

pub trait FirstLast {
    type Ret;

    fn first(&self) -> &Self::Ret;
    fn last(&self) -> &Self::Ret;

    fn first_mut(&mut self) -> &mut Self::Ret;
    fn last_mut(&mut self) -> &mut Self::Ret;
}

impl<T> FirstLast for Vec<T> {
    type Ret = T;

    fn first(&self) -> &T {
        &self[0]
    }

    fn last(&self) -> &T {
        let len = self.len();
        &self[len - 1]
    }

    fn first_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn last_mut(&mut self) -> &mut T {
        let len = self.len();
        &mut self[len - 1]
    }
}

pub trait FromChar {
    type Ret;

    fn from_char(c: char) -> Self::Ret;
}
