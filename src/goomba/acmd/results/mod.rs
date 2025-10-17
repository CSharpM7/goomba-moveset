//mod win1; //Team Win
//mod win2; //GoombaTower
mod win3; //MVP (Mario Party)

pub fn install(agent: &mut smashline::Agent) {
    //win1::install(agent);
    //win2::install(agent);
    win3::install(agent);
}
