mod attack;
mod attackair;
mod finalsmash;
mod landing;
mod entry_appeal;
mod specials;

pub fn install(agent: &mut smashline::Agent) {
    attack::install(agent);
    attackair::install(agent);
	finalsmash::install(agent);
	landing::install(agent);
    entry_appeal::install(agent);
	specials::install(agent);
}