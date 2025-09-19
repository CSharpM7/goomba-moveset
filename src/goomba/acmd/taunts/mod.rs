mod appealhi;
mod appeals;
mod appeallw;

pub fn install(agent: &mut smashline::Agent) {
    appealhi::install(agent);
    appeals::install(agent);
    appeallw::install(agent);
}
