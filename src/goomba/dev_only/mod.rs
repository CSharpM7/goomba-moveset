mod testing;

mod reveal_intro;
mod cinema_gameplay;

pub fn install(agent: &mut smashline::Agent) {
    testing::install(agent);
    cinema_gameplay::install(agent);
    reveal_intro::install(agent);
}