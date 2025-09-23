mod attack;
mod specials;
mod landing;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    attack::install(agent);
	specials::install(agent);
	landing::install(agent);
    appeal::install(agent);
}