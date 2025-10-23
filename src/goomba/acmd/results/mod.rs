//mod win1; //Team Win
mod win2; //GoombaTower
mod win3; //MVP (Mario Party) 
use crate::imports::imports_acmd::empty_acmd;

pub fn install(agent: &mut smashline::Agent) {
    //win1::install(agent);
    win2::install(agent);
    win3::install(agent);

	//agent.acmd("effect_win3", empty_acmd, smashline::Priority::Default);
	//agent.acmd("sound_win3", empty_acmd, smashline::Priority::Default);
}