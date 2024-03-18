mod tilts;
mod smashes;
mod aerials;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
}