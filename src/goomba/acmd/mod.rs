mod tilts;
mod smashes;
mod aerials;
mod throws;
mod specials;
//mod finalsmash;

//mod entry_results_appeal;

mod movement;
//mod misc;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    throws::install(agent);
    specials::install(agent);
    //finalsmash::install(agent);

    //entry_results_appeal::install(agent);

    movement::install(agent);
    //misc::install(agent);
}