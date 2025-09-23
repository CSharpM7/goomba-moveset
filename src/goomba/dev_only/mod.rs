//mod reveal_intro;
mod gameplay;

pub fn install(agent: &mut smashline::Agent) {
    gameplay::install(agent);
    //reveal_intro::install(agent);
}