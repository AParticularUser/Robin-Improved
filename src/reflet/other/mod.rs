mod grab;
mod taunts;
mod resurrection;

pub fn install(agent: &mut smashline::Agent) {
    grab::install(agent);
    taunts::install(agent);
    resurrection::install(agent);
}