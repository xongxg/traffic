#[derive(Debug, Eq, PartialEq)]
struct Green;

#[derive(Debug, Eq, PartialEq)]
struct Yellow;

#[derive(Debug, Eq, PartialEq)]
struct Red;

#[derive(Debug, Eq, PartialEq)]
struct TrafficLight<N, E> {
    ns_state: N,
    ew_state: E,
}

impl<N, E> TrafficLight<N, E> {
    fn new(ns_state: N, ew_state: E) -> Self {
        TrafficLight { ns_state, ew_state }
    }
}

impl Default for TrafficLight<Green, Red> {
    fn default() -> Self {
        TrafficLight {
            ns_state: Green,
            ew_state: Red,
        }
    }
}

impl From<TrafficLight<Green, Red>> for TrafficLight<Yellow, Red> {
    fn from(_value: TrafficLight<Green, Red>) -> Self {
        TrafficLight {
            ns_state: Yellow,
            ew_state: Red,
        }
    }
}

impl From<TrafficLight<Yellow, Red>> for TrafficLight<Red, Green> {
    fn from(_val: TrafficLight<Yellow, Red>) -> TrafficLight<Red, Green> {
        TrafficLight {
            ns_state: Red,
            ew_state: Green,
        }
    }
}

impl From<TrafficLight<Red, Green>> for TrafficLight<Red, Yellow> {
    fn from(_val: TrafficLight<Red, Green>) -> TrafficLight<Red, Yellow> {
        TrafficLight {
            ns_state: Red,
            ew_state: Yellow,
        }
    }
}

impl From<TrafficLight<Red, Yellow>> for TrafficLight<Green, Red> {
    fn from(_val: TrafficLight<Red, Yellow>) -> TrafficLight<Green, Red> {
        TrafficLight {
            ns_state: Green,
            ew_state: Red,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TrafficLightWrapper {
    G1R2(TrafficLight<Green, Red>),
    Y1R2(TrafficLight<Yellow, Red>),
    R1G2(TrafficLight<Red, Green>),
    R1Y2(TrafficLight<Red, Yellow>),
}

impl TrafficLightWrapper {
    pub fn step(&self) -> Self {
        match self {
            TrafficLightWrapper::G1R2(val) => TrafficLightWrapper::Y1R2(val.into()),
            TrafficLightWrapper::Y1R2(val) => TrafficLightWrapper::R1G2(val.into()),
            TrafficLightWrapper::R1G2(val) => TrafficLightWrapper::R1Y2(val.into()),
            TrafficLightWrapper::R1Y2(val) => TrafficLightWrapper::G1R2(val.into()),
        }
    }

    pub fn new() -> Self {
        TrafficLightWrapper::G1R2(TrafficLight::default())
    }
}
