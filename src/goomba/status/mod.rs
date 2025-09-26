mod attack;
mod attackair;
mod specials;
mod finalsmash;
mod landing;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    attack::install(agent);
    attackair::install(agent);
	specials::install(agent);
	finalsmash::install(agent);
    appeal::install(agent);
	landing::install(agent);
    appeal::install(agent);
}