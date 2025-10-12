mod start;
mod dash;
mod end;
mod vortex;

pub fn install(agent: &mut smashline::Agent) {
    start::install(agent);
    dash::install(agent);
    end::install(agent);
    vortex::install(agent);
}
