mod attacks4;
mod attackhi4;
mod attacklw4;

pub fn install(agent: &mut smashline::Agent) {
    attacks4::install(agent);
    attackhi4::install(agent);
    attacklw4::install(agent);
}
