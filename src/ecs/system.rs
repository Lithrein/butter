use super::{
    query::{Description, Query},
    Ecs,
};

pub trait System<'e, P> {
    fn run(&'e self, ecs: &'e Ecs);
}

pub trait Injectable<'e> {
    fn inject(ecs: &'e Ecs) -> Self;
}

impl<'e, QD> Injectable<'e> for Query<'e, QD>
where
    QD: for<'d> Description<'d>,
{
    fn inject(ecs: &'e Ecs) -> Self {
        Self::new(ecs)
    }
}

macro_rules! impl_system_for_fun {
    ($($p:tt,)*) => {
        impl<'e, FN, $($p,)*> System<'e, ($($p,)*)> for FN
        where
            FN: Fn($(&$p,)*),
            $($p: Injectable<'e>,)*
        {
            fn run(&'e self, ecs: &'e Ecs) {
                (self)($(&$p::inject(ecs),)*);
            }
        }

    };
}

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

#[cfg(test)]
mod tests {
    use crate::ecs::{query::Query, Ecs};

    #[derive(Debug, PartialEq, Eq)]
    struct Player;
    #[derive(Debug, PartialEq, Eq)]
    struct Enemy;
    #[derive(Debug, PartialEq, Eq)]
    struct Health(i16);

    fn instakill_all_enemies_and_heal_player(
        players_with_health: &Query<(&Player, &mut Health)>,
        enemies_with_health: &Query<(&Enemy, &mut Health)>,
    ) {
        for (_, health) in players_with_health.iter() {
            health.0 = 10;
        }

        for (_, health) in enemies_with_health.iter() {
            health.0 = 0;
        }
    }

    #[test]
    fn simple_system() {
        let mut ecs = Ecs::new();
        ecs.insert((Player, Health(5)));
        ecs.insert((Enemy, Health(2)));
        ecs.insert((Enemy, Health(1)));

        ecs.run_system(&instakill_all_enemies_and_heal_player);

        for (_, health) in ecs.query::<(&Player, &Health)>() {
            assert_eq!(health, &Health(10));
        }

        for (_, health) in ecs.query::<(&Enemy, &Health)>() {
            assert_eq!(health, &Health(0));
        }
    }
}
