mod tilts;
mod smashes;
mod aerials;
mod throws;
mod specials;
mod finalsmash;

mod movement;

mod taunts;
mod entry;
mod results;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent); 
    smashes::install(agent);
    aerials::install(agent);
    throws::install(agent);
    specials::install(agent);
    finalsmash::install(agent);

    movement::install(agent);

    taunts::install(agent);
    entry::install(agent);
    results::install(agent);
}