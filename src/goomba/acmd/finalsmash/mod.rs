mod start;
mod dash;
//mod attack;
//mod hit_return;
mod end;
mod vortex;

pub fn install(agent: &mut smashline::Agent) {
    start::install(agent);
    dash::install(agent);
    //attack::install(agent);
    //hit_return::install(agent);
    end::install(agent);
    vortex::install(agent);
}
