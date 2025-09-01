mod attacks4;

mod specials;

pub fn install(agent: &mut smashline::Agent) {
    attacks4::install(agent);

	specials::install(agent);
}