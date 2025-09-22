use crate::imports::imports_acmd::*;

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("effect_win2", effect_win2, Priority::Default);
	agent.acmd("sound_win2", sound_win2, Priority::Default);
}
