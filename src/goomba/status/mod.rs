mod attack;
mod specials;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    attack::install(agent);
	specials::install(agent);
    appeal::install(agent);
}