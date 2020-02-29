use crate::cells::event_handler::Target;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Action {
    Follow { target: Target },
    Flee { target: Target },
    Divide,
}
