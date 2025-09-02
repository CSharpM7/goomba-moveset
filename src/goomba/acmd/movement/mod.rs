mod walk;
mod dash_run_turn;

mod jump;
mod step;
mod cliff;

mod down;
mod escape_passive_slip;

mod attacks;

pub fn install(agent: &mut smashline::Agent) {
    //walk::install(agent);
    dash_run_turn::install(agent);

    //jump::install(agent);
    step::install(agent);
    //cliff::install(agent);

    //down::install(agent);
    //escape_passive_slip::install(agent);
    attacks::install(agent);
}
