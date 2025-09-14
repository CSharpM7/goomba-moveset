mod attack;

mod specials;

pub fn install(agent: &mut smashline::Agent) {
    attack::install(agent);

	specials::install(agent);
}