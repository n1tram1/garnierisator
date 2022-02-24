use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SimulationError {
    SignalDoesntExist(String)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SignalState {
    High,
    Low,
    Unknown,
}

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
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

    pub fn update_with(&mut self, other: Signals) {
        for (name, signal) in other.signals.into_iter() {
            if let Some(value) =  self.signals.get_mut(&name) {
                *value = signal;
            } else {
                self.signals.insert(name, signal);
            }
        }
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
