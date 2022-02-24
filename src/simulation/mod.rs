use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SignalState {
    High,
    Low,
    Unknown,
}

pub struct Signal {
    name: String,
    state: SignalState,
}

impl Signal {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            state: SignalState::Unknown,
        }
    }

    pub fn set_low(&mut self) {
        self.state = SignalState::Low;
    }

    pub fn set_high(&mut self) {
        self.state = SignalState::High;
    }
}

pub struct Signals {
    signals: HashMap<String, Signal>,
}

impl Signals {
    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
        }
    }

    pub fn add_signal(&mut self, signal: Signal) {
        self.signals.insert(
            signal.name.clone(),
            signal
        );
    }

    pub fn get(&self, name: &str) -> SignalState {
        match self.signals.get(name) {
            Some(signal) => signal.state,
            None => SignalState::Unknown,
        }
    }
}

pub struct SignalsBuilder {
    signals: HashMap<String, Signal>,
}

impl SignalsBuilder {
    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
        }
    }

    pub fn add_signal(mut self, name: &str, state: SignalState) -> Self {
        let signal = Signal { name: name.to_string(), state };
        self.signals.insert(name.to_string(), signal);

        self
    }

    pub fn build(self) -> Signals {
        Signals {
            signals: self.signals
        }
    }
}

pub trait Simulable {
    fn stim(&self, inputs: Signals) -> Signals;
}
