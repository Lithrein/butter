use std::marker::PhantomData;

use super::Ecs;

pub trait Description<'e> {
    type Item;

    fn fetch(ecs: &'e Ecs, index: usize) -> Option<Self::Item>;
}

pub struct Query<'e, D>
where
    D: for<'d> Description<'d>,
{
    ecs: &'e Ecs,
    _marker: PhantomData<D>,
}

impl<'e, D> Query<'e, D>
where
    D: for<'d> Description<'d>,
{
    pub fn new(ecs: &'e Ecs) -> Self {
        Self {
            ecs,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> Iter<D> {
        Iter::new(self.ecs)
    }
}

impl<'a, T: 'static> Description<'a> for &T {
    type Item = &'a T;

    fn fetch(ecs: &'a Ecs, index: usize) -> Option<Self::Item> {
        ecs.component_at_index::<T>(index)
    }
}

impl<'a, T: 'static> Description<'a> for &mut T {
    type Item = &'a mut T;

    fn fetch(ecs: &'a Ecs, index: usize) -> Option<Self::Item> {
        ecs.component_mut_at_index::<T>(index)
    }
}

macro_rules! impl_query_description_for_tuple {
    ($($t:tt,)*) => {
        impl<'a, $($t),*> Description<'a> for ($($t,)*)
        where
            $($t: 'static + Description<'a>,)*
        {
            type Item = ($($t::Item,)*);

            fn fetch(ecs: &'a Ecs, index: usize) -> Option<Self::Item> {
                Some(($($t::fetch(ecs, index)?,)*))
            }
        }
    };
}

impl_query_description_for_tuple!(A,);
impl_query_description_for_tuple!(A, B,);
impl_query_description_for_tuple!(A, B, C,);
impl_query_description_for_tuple!(A, B, C, D,);
impl_query_description_for_tuple!(A, B, C, D, E,);
impl_query_description_for_tuple!(A, B, C, D, E, F,);
impl_query_description_for_tuple!(A, B, C, D, E, F, G,);
impl_query_description_for_tuple!(A, B, C, D, E, F, G, H,);
impl_query_description_for_tuple!(A, B, C, D, E, F, G, H, I,);
impl_query_description_for_tuple!(A, B, C, D, E, F, G, H, I, J,);
impl_query_description_for_tuple!(A, B, C, D, E, F, G, H, I, J, K,);
impl_query_description_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L,);
impl_query_description_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M,);
impl_query_description_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N,);

pub struct Iter<'a, Q>
where
    Q: Description<'a>,
{
    ecs: &'a Ecs,
    current_index: usize,
    _marker: PhantomData<&'a Q>,
}

impl<'a, Q> Iter<'a, Q>
where
    Q: Description<'a>,
{
    pub(crate) fn new(ecs: &'a Ecs) -> Self {
        Self {
            ecs,
            current_index: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, Q> Iterator for Iter<'a, Q>
where
    Q: Description<'a>,
{
    type Item = Q::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index > self.ecs.next_index {
            return None;
        }

        let next = Q::fetch(self.ecs, self.current_index);
        self.current_index += 1;
        next
    }
}
