pub mod special_hi;
mod special_lw;
pub mod special_n;
mod special_s;

pub fn install(agent: &mut smashline::Agent) {
    special_hi::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}