use super::{
    query::{Description, Query},
    Ecs,
};

pub trait System<'e, QD> {
    fn run(&self, ecs: &'e Ecs);
}

impl<'e, F, QD> System<'e, QD> for F
where
    F: Fn(&Query<QD>),
    QD: for<'d> Description<'d>,
{
    fn run(&self, ecs: &'e Ecs) {
        (self)(&Query::new(ecs));
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::{query::Query, Ecs};

    #[derive(Debug, PartialEq, Eq)]
    struct Health(i16);

    fn restore_health_system(query: &Query<&mut Health>) {
        for health in query.iter() {
            health.0 = 10;
        }
    }

    #[test]
    fn simple_system() {
        let mut ecs = Ecs::new();
        ecs.insert((Health(5),));
        ecs.insert((Health(2),));
        ecs.insert((Health(1),));

        ecs.run_system(&restore_health_system);

        for health in ecs.query::<&Health>() {
            assert_eq!(health, &Health(10));
        }
    }
}
