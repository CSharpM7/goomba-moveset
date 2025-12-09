mod tilts;
mod smashes;
mod aerials;
mod throws;
mod finalsmash;
mod specials;

mod movement;
mod entry_appeal;
mod results;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent); 
    smashes::install(agent);
    aerials::install(agent);
    throws::install(agent);
    finalsmash::install(agent);
    specials::install(agent);

    movement::install(agent);
    entry_appeal::install(agent);
    results::install(agent);
}