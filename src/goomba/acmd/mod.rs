mod tilts;
mod smashes;
mod aerials;
mod throws;
mod specials;
//mod finalsmash;

mod movement;
//mod misc;

mod taunts;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    throws::install(agent);
    specials::install(agent);
    //finalsmash::install(agent);

    movement::install(agent);
    //misc::install(agent);

    taunts::install(agent);
}