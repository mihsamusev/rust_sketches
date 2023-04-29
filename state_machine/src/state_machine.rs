#[derive(Debug, PartialEq)]
struct BottleFillingMachine<S> {
    value: usize,
    state: S,
}

#[derive(Debug, PartialEq)]
struct Waiting {
    time: std::time::Duration,
}

#[derive(Debug, PartialEq)]
struct Filling {
    rate: usize,
}

#[derive(Debug, PartialEq)]
struct Done;

enum State {
    Waiting(Waiting),
    Filling(Filling),
    Done,
}

impl BottleFillingMachine<Waiting> {
    fn new(value: usize) -> Self {
        BottleFillingMachine {
            value,
            state: Waiting {
                time: std::time::Duration::new(0, 0),
            },
        }
    }
}
impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(machine_state: BottleFillingMachine<Waiting>) -> Self {
        BottleFillingMachine {
            value: machine_state.value,
            state: Filling { rate: 1 },
        }
    }
}

impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(machine_state: BottleFillingMachine<Filling>) -> Self {
        BottleFillingMachine {
            value: machine_state.value,
            state: Done,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_waiting_machine_transition_to_next_state() {
        let machine = BottleFillingMachine::new(10);
        assert_eq!(
            BottleFillingMachine::<Filling> {
                value: 10,
                state: Filling { rate: 1 }
            },
            machine.into()
        );
    }
}
