pub trait All<T> {
    fn all(self) -> T;
}

impl<'a, A> All<Option<&'a A>> for (&'a Option<A>) {
    fn all(self) -> Option<&'a A> {
        match self {
            &Some(ref a) => Some(a),
            _ => None
        }
    }
}

impl<'a, A> All<Option<&'a mut A>> for (&'a mut Option<A>) {
    fn all(self) -> Option<&'a mut A> {
        match self {
            &mut Some(ref mut a) => Some(a),
            _ => None
        }
    }
}

impl<'a, 'b, A, B> All<Option<(&'a A, &'b B)>> for (&'a Option<A>, &'b Option<B>) {
    fn all(self) -> Option<(&'a A, &'b B)> {
        match self {
            (&Some(ref a), &Some(ref b)) => Some((a, b)),
            _ => None
        }
    }
}

impl<'a, 'b, A, B> All<Option<(&'a A, &'b mut B)>> for (&'a Option<A>, &'b mut Option<B>) {
    fn all(self) -> Option<(&'a A, &'b mut B)> {
        match self {
            (&Some(ref a), &mut Some(ref mut b)) => Some((a, b)),
            _ => None
        }
    }
}

impl<'a, 'b, A, B> All<Option<(&'a mut A, &'b B)>> for (&'a mut Option<A>, &'b Option<B>) {
    fn all(self) -> Option<(&'a mut A, &'b B)> {
        match self {
            (&mut Some(ref mut a), &Some(ref b)) => Some((a, b)),
            _ => None
        }
    }
}

impl<'a, 'b, A, B> All<Option<(&'a mut A, &'b mut B)>> for (&'a mut Option<A>, &'b mut Option<B>) {
    fn all(self) -> Option<(&'a mut A, &'b mut B)> {
        match self {
            (&mut Some(ref mut a), &mut Some(ref mut b)) => Some((a, b)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a A, &'b B, &'c C)>> for ((&'a Option<A>, &'b Option<B>), &'c Option<C>) {
    fn all(self) -> Option<(&'a A, &'b B, &'c C)> {
        match self {
            ((&Some(ref a), &Some(ref b)), &Some(ref c)) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a mut A, &'b B, &'c C)>> for ((&'a mut Option<A>, &'b Option<B>), &'c Option<C>) {
    fn all(self) -> Option<(&'a mut A, &'b B, &'c C)> {
        match self {
            ((&mut Some(ref mut a), &Some(ref b)), &Some(ref c)) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a A, &'b mut B, &'c C)>> for ((&'a Option<A>, &'b mut Option<B>), &'c Option<C>) {
    fn all(self) -> Option<(&'a A, &'b mut B, &'c C)> {
        match self {
            ((&Some(ref a), &mut Some(ref mut b)), &Some(ref c)) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a A, &'b B, &'c mut C)>> for ((&'a Option<A>, &'b Option<B>), &'c mut Option<C>) {
    fn all(self) -> Option<(&'a A, &'b B, &'c mut C)> {
        match self {
            ((&Some(ref a), &Some(ref b)), &mut Some(ref mut c)) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a mut A, &'b mut B, &'c C)>> for ((&'a mut Option<A>, &'b mut Option<B>), &'c Option<C>) {
    fn all(self) -> Option<(&'a mut A, &'b mut B, &'c C)> {
        match self {
            ((&mut Some(ref mut a), &mut Some(ref mut b)), &Some(ref c)) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a mut A, &'b B, &'c mut C)>> for ((&'a mut Option<A>, &'b Option<B>), &'c mut Option<C>) {
    fn all(self) -> Option<(&'a mut A, &'b B, &'c mut C)> {
        match self {
            ((&mut Some(ref mut a), &Some(ref b)), &mut Some(ref mut c)) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a A, &'b mut B, &'c mut C)>> for ((&'a Option<A>, &'b mut Option<B>), &'c mut Option<C>) {
    fn all(self) -> Option<(&'a A, &'b mut B, &'c mut C)> {
        match self {
            ((&Some(ref a), &mut Some(ref mut b)), &mut Some(ref mut c)) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a mut A, &'b mut B, &'c mut C)>> for ((&'a mut Option<A>, &'b mut Option<B>), &'c mut Option<C>) {
    fn all(self) -> Option<(&'a mut A, &'b mut B, &'c mut C)> {
        match self {
            ((&mut Some(ref mut a), &mut Some(ref mut b)), &mut Some(ref mut c)) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, 'd, A, B, C, D>
All<Option<(&'a A, &'b B, &'c C, &'d D)>>
for (&'a Option<A>, &'b Option<B>, &'c Option<C>, &'d Option<D>) {
    fn all(self) -> Option<(&'a A, &'b B, &'c C, &'d D)> {
        match self {
            (&Some(ref a), &Some(ref b), &Some(ref c), &Some(ref d)) => Some((a, b, c, d)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, 'd, A, B, C, D>
All<Option<(&'a mut A, &'b mut B, &'c mut C, &'d mut D)>>
for (&'a mut Option<A>, &'b mut Option<B>, &'c mut Option<C>, &'d mut Option<D>) {
    fn all(self) -> Option<(&'a mut A, &'b mut B, &'c mut C, &'d mut D)> {
        match self {
            (&mut Some(ref mut a), &mut Some(ref mut b), &mut Some(ref mut c), &mut Some(ref mut d)) => Some((a, b, c, d)),
            _ => None
        }
    }
}

impl<'a, 'b, A, B> All<Option<(&'a A, &'b B)>> for (&'a Option<A>, &'b B) {
    fn all(self) -> Option<(&'a A, &'b B)> {
        match self {
            (&Some(ref a), ref b) => Some((a, b)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a A, &'b B, &'c C)>> for ((&'a Option<A>, &'b Option<B>), &'c C) {
    fn all(self) -> Option<(&'a A, &'b B, &'c C)> {
        match self {
            ((&Some(ref a), &Some(ref b)), ref c) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a mut A, &'b B, &'c C)>> for ((&'a mut Option<A>, &'b Option<B>), &'c C) {
    fn all(self) -> Option<(&'a mut A, &'b B, &'c C)> {
        match self {
            ((&mut Some(ref mut a), &Some(ref b)), ref c) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a A, &'b mut B, &'c C)>> for ((&'a Option<A>, &'b mut Option<B>), &'c C) {
    fn all(self) -> Option<(&'a A, &'b mut B, &'c C)> {
        match self {
            ((&Some(ref a), &mut Some(ref mut b)), ref c) => Some((a, b, c)),
            _ => None
        }
    }
}

impl<'a, 'b, 'c, A, B, C> All<Option<(&'a mut A, &'b mut B, &'c C)>> for ((&'a mut Option<A>, &'b mut Option<B>), &'c C) {
    fn all(self) -> Option<(&'a mut A, &'b mut B, &'c C)> {
        match self {
            ((&mut Some(ref mut a), &mut Some(ref mut b)), ref c) => Some((a, b, c)),
            _ => None
        }
    }
}