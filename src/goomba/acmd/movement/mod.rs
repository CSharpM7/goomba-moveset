//mod walk;
mod dash_run_turn;
mod step;

mod misc;

mod attacks;

pub fn install(agent: &mut smashline::Agent) {
    //walk::install(agent);
    dash_run_turn::install(agent);
    step::install(agent);

    misc::install(agent);
    
    attacks::install(agent);
}
