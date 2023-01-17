use std::{marker::PhantomData, ops::DerefMut};

use crate::commands::CommandQueue;

use super::{
    query::{Description, Query},
    Ecs,
};

pub trait System: 'static {
    fn run(&mut self, ecs: &Ecs);
    fn command_queue(&mut self) -> &mut CommandQueue;
}

impl System for Box<dyn System> {
    fn run(&mut self, ecs: &Ecs) {
        self.deref_mut().run(ecs);
    }

    fn command_queue(&mut self) -> &mut CommandQueue {
        self.deref_mut().command_queue()
    }
}

macro_rules! impl_system_for_fun {
    ($($p:tt,)*) => {
        impl<FN, $($p),*> System for Function<FN, ($($p,)*)>
        where
            FN: 'static + for<'ecs> FnMut(&mut CommandQueue, $(&$p::Type<'ecs>,)*),
            $($p: 'static + Parameter,)*
        {
            #[allow(unused_variables)]
            fn run(&mut self, ecs: &Ecs) {
                (self.system_fn)(&mut self.command_queue, $(&$p::fetch(ecs),)*)
            }

            fn command_queue(&mut self) -> &mut CommandQueue {
                &mut self.command_queue
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
            FN: 'static + FnMut(&mut CommandQueue, $(&$t,)*),
            $($t: Parameter,)*
        {
            type SystemType = Function<FN, ($($t,)*)>;

            fn into_system(self) -> Self::SystemType {
                Function {
                    command_queue: CommandQueue::new(),
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
    command_queue: CommandQueue,
    system_fn: F,
    _marker: PhantomData<P>,
}

#[cfg(test)]
mod tests {
    use crate::query::Query;

    use super::*;

    #[test]
    fn system_with_single_query() {
        #[derive(Debug, PartialEq, Eq)]
        struct Player;
        #[derive(Debug, PartialEq, Eq)]
        struct Health(i16);

        fn restore_player_health(_: &mut CommandQueue, query: &Query<(&Player, &mut Health)>) {
            for (_, health) in query.iter() {
                health.0 = 10;
            }
        }

        let mut ecs = Ecs::new();
        ecs.insert((Player, Health(8)));
        ecs.insert((Player, Health(5)));
        ecs.run_single_system(&mut restore_player_health.into_system());

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
            _: &mut CommandQueue,
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
        ecs.run_single_system(&mut restore_player_health.into_system());

        for (_, health) in ecs.query::<(&Player, &Health)>() {
            assert_eq!(health, &Health(10));
        }

        for (_, health) in ecs.query::<(&Enemy, &Health)>() {
            assert_eq!(health, &Health(0));
        }
    }

    #[test]
    fn system_inserting_entities() {
        #[derive(Debug, PartialEq, Eq)]
        struct Player;
        #[derive(Debug, PartialEq, Eq)]
        struct Enemy;
        #[derive(Debug, PartialEq, Eq)]
        struct Health(i16);
        fn insert_entities(command_queue: &mut CommandQueue) {
            command_queue.insert((Player, Health(10)));
            command_queue.insert((Enemy, Health(8)));
        }

        let mut ecs = Ecs::new();
        assert_eq!(ecs.entity_count(), 0);
        ecs.run_single_system(&mut insert_entities.into_system());
        assert_eq!(ecs.entity_count(), 2);
    }
}
