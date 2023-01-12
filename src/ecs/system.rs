use std::marker::PhantomData;

use super::{
    query::{Description, Query},
    Ecs,
};

pub trait System: 'static {
    fn run(&mut self, ecs: &Ecs);
}

macro_rules! impl_system_for_fun {
    ($($p:tt,)*) => {
        impl<FN, $($p),*> System for Function<FN, ($($p,)*)>
        where
            FN: 'static + for<'ecs> FnMut($(&$p::Type<'ecs>,)*),
            $($p: 'static + Parameter,)*
        {
            #[allow(unused_variables)]
            fn run(&mut self, ecs: &Ecs) {
                (self.system_fn)($(&$p::fetch(ecs),)*)
            }
        }
    };
}

impl_system_for_fun!();
impl_system_for_fun!(A,);
impl_system_for_fun!(A, B,);
impl_system_for_fun!(A, B, C,);
impl_system_for_fun!(A, B, C, D,);
impl_system_for_fun!(A, B, C, D, E,);
impl_system_for_fun!(A, B, C, D, E, F,);
impl_system_for_fun!(A, B, C, D, E, F, G,);
impl_system_for_fun!(A, B, C, D, E, F, G, H,);
impl_system_for_fun!(A, B, C, D, E, F, G, H, I,);
impl_system_for_fun!(A, B, C, D, E, F, G, H, I, J,);
impl_system_for_fun!(A, B, C, D, E, F, G, H, I, J, K,);
impl_system_for_fun!(A, B, C, D, E, F, G, H, I, J, K, L,);
impl_system_for_fun!(A, B, C, D, E, F, G, H, I, J, K, L, M,);
impl_system_for_fun!(A, B, C, D, E, F, G, H, I, J, K, L, M, N,);

pub trait Parameter {
    type Type<'ecs>;
    fn fetch(ecs: &Ecs) -> Self::Type<'_>;
}

macro_rules! impl_parameter_for_tuple {
    ($($t:tt,)*) => {
        impl<$($t,)*> Parameter for ($($t,)*) where
        $($t: Parameter,)* {
            type Type<'ecs> = ($($t::Type<'ecs>,)*);
            #[allow(unused_variables)]
            #[allow(clippy::unused_unit)]
            fn fetch(ecs: &Ecs) -> Self::Type<'_> {
                ($($t::fetch(ecs),)*)
            }
        }
    };
}

impl_parameter_for_tuple!();
impl_parameter_for_tuple!(A,);
impl_parameter_for_tuple!(A, B,);
impl_parameter_for_tuple!(A, B, C,);
impl_parameter_for_tuple!(A, B, C, D,);
impl_parameter_for_tuple!(A, B, C, D, E,);
impl_parameter_for_tuple!(A, B, C, D, E, F,);
impl_parameter_for_tuple!(A, B, C, D, E, F, G,);
impl_parameter_for_tuple!(A, B, C, D, E, F, G, H,);
impl_parameter_for_tuple!(A, B, C, D, E, F, G, H, I,);
impl_parameter_for_tuple!(A, B, C, D, E, F, G, H, I, J,);
impl_parameter_for_tuple!(A, B, C, D, E, F, G, H, I, J, K,);
impl_parameter_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L,);
impl_parameter_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M,);
impl_parameter_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N,);

impl<D> Parameter for Query<'_, D>
where
    D: for<'d> Description<'d>,
{
    type Type<'ecs> = Query<'ecs, D>;
    fn fetch(ecs: &Ecs) -> Self::Type<'_> {
        Query::new(ecs)
    }
}

pub trait Into<P>
where
    P: Parameter,
{
    type SystemType;
    fn into_system(self) -> Self::SystemType;
}

macro_rules! impl_into_for_fun {
    ($($t:tt,)*) => {
        impl<FN, $($t,)*> Into<($($t,)*)> for FN
        where
            FN: 'static + FnMut($(&$t,)*),
            $($t: Parameter,)*
        {
            type SystemType = Function<FN, ($($t,)*)>;

            fn into_system(self) -> Self::SystemType {
                Function {
                    system_fn: self,
                    _marker: PhantomData,
                }
            }
        }
    };
}

impl_into_for_fun!();
impl_into_for_fun!(A,);
impl_into_for_fun!(A, B,);
impl_into_for_fun!(A, B, C,);
impl_into_for_fun!(A, B, C, D,);
impl_into_for_fun!(A, B, C, D, E,);
impl_into_for_fun!(A, B, C, D, E, F,);
impl_into_for_fun!(A, B, C, D, E, F, G,);
impl_into_for_fun!(A, B, C, D, E, F, G, H,);
impl_into_for_fun!(A, B, C, D, E, F, G, H, I,);
impl_into_for_fun!(A, B, C, D, E, F, G, H, I, J,);
impl_into_for_fun!(A, B, C, D, E, F, G, H, I, J, K,);
impl_into_for_fun!(A, B, C, D, E, F, G, H, I, J, K, L,);
impl_into_for_fun!(A, B, C, D, E, F, G, H, I, J, K, L, M,);
impl_into_for_fun!(A, B, C, D, E, F, G, H, I, J, K, L, M, N,);

pub struct Function<F, P>
where
    P: Parameter,
{
    system_fn: F,
    _marker: PhantomData<P>,
}

#[cfg(test)]
mod tests {
    use crate::ecs::query::Query;

    use super::*;

    #[test]
    fn system_with_single_query() {
        #[derive(Debug, PartialEq, Eq)]
        struct Player;
        #[derive(Debug, PartialEq, Eq)]
        struct Health(i16);

        fn restore_player_health(query: &Query<(&Player, &mut Health)>) {
            for (_, health) in query.iter() {
                health.0 = 10;
            }
        }

        let mut ecs = Ecs::new();
        ecs.insert((Player, Health(8)));
        ecs.insert((Player, Health(5)));
        ecs.run_system(&mut restore_player_health.into_system());

        for (_, health) in ecs.query::<(&Player, &Health)>() {
            assert_eq!(health, &Health(10));
        }
    }

    #[test]
    fn system_with_multiple_queries() {
        #[derive(Debug, PartialEq, Eq)]
        struct Player;
        #[derive(Debug, PartialEq, Eq)]
        struct Enemy;
        #[derive(Debug, PartialEq, Eq)]
        struct Health(i16);

        fn restore_player_health(
            query: &Query<(&Player, &mut Health)>,
            query2: &Query<(&Enemy, &mut Health)>,
        ) {
            for (_, health) in query.iter() {
                health.0 = 10;
            }

            for (_, health) in query2.iter() {
                health.0 = 0;
            }
        }

        let mut ecs = Ecs::new();
        ecs.insert((Player, Health(8)));
        ecs.insert((Player, Health(5)));
        ecs.insert((Enemy, Health(12)));
        ecs.insert((Enemy, Health(9)));
        ecs.run_system(&mut restore_player_health.into_system());

        for (_, health) in ecs.query::<(&Player, &Health)>() {
            assert_eq!(health, &Health(10));
        }

        for (_, health) in ecs.query::<(&Enemy, &Health)>() {
            assert_eq!(health, &Health(0));
        }
    }
}
