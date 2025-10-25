/*mod attack;
mod attackair;
mod specials;
mod finalsmash;
mod landing;*/
mod entry_appeal;

pub fn install(agent: &mut smashline::Agent) {
    /*attack::install(agent);
    attackair::install(agent);
	specials::install(agent);
	finalsmash::install(agent);
	landing::install(agent);*/
    entry_appeal::install(agent);
}