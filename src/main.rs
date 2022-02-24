#![feature(map_try_insert)]

mod blif;
mod simulation;

fn main() {
    blif::parser::parse("");
}

#[cfg(test)]
mod tests {
    use crate::blif::*;
    use crate::simulation::*;

    #[test]
    fn test_pipeline() {
        let smol = include_str!("../fixtures/smol.blif");
        let blif = parser::parse(smol);

        let res = blif.stim(
            SignalsBuilder::new()
                .add_signal("i_A", SignalState::Low)
                .add_signal("i_B", SignalState::Low)
                .build()
        );
        assert_eq!(res.get("o_led"), SignalState::Low);

        let res = blif.stim(
            SignalsBuilder::new()
                .add_signal("i_A", SignalState::High)
                .add_signal("i_B", SignalState::Low)
                .build()
        );
        assert_eq!(res.get("o_led"), SignalState::High);
    }
}
