mod win1; //Team Win (Mario Party)
mod win2; //GoombaTower
mod win3; //MVP (Sluggers) 

pub fn install(agent: &mut smashline::Agent) {
    win1::install(agent);
    win2::install(agent);
    win3::install(agent);
}