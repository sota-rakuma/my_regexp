#[derive(Debug, PartialEq, Eq, Clone)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub struct ListIter<T> {
    current: Option<Box<List<T>>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> ListIter<T>
    where
        T: Clone,
    {
        ListIter {
            current: match self.clone() {
                List::Nil => {None},
                v => Some(Box::new(v))
            },
        }
    }
}

impl<T> Iterator for ListIter<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            Some(boxed_node) => match *boxed_node {
                List::Cons(value, next) => {
                    self.current = Some(next);
                    Some(value)
                }
                List::Nil => {
                    None
                }
            },
            None => None,
        }
    }
}

impl<T> List<T> {
    pub fn new(v: Option<T>) -> List<T> {
        if v.is_none() {
            List::Nil
        } else {
            List::Cons(v.unwrap(), Box::new(List::Nil))
        }
    }

    pub fn from<I: DoubleEndedIterator<Item = T>>(iter: I) -> List<T> {
        let iter = iter.rev();
        Self::from_rev(iter)
    }

    pub fn from_rev<I: IntoIterator<Item = T> + Iterator<Item = T>>(mut iter: I) -> List<T> {
        let mut list = List::<T>::new(None);
        while let Some(v) = iter.next() {
            list = list.prepend(v);
        }
        list
    }


    pub fn prepend(self, elem: T) -> List<T> {
        List::Cons(elem, Box::new(self))
    }
}

#[macro_export]
macro_rules! list {
    ($exp:expr) => {
        $crate::utils::list::List::new(Some($exp))
    };
    () => {
        $crate::utils::list::List::new(None)
    };
}

#[cfg(test)]
mod test {
    use crate::utils::list::List;

    #[test]
    fn list_iter_test() {
        let list = List::from([1, 2, 3].into_iter());
        let mut iter = list.iter();
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(None, iter.next());

        let list = list!(1);
        let mut iter = list.iter();
        assert_eq!(Some(1), iter.next());
        assert_eq!(None, iter.next());

        let list: List::<i32> = list!();
        assert_eq!(None, list.iter().next());
    }
}