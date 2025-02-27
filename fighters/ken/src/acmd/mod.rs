use super::*;
mod aerials;
mod tilts;
mod other;
mod smashes;
mod specials;
mod throws;
mod ground;
mod finals;

pub fn install() {
    aerials::install();
    tilts::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    ground::install();
    finals::install();
}
