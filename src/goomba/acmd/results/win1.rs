use crate::imports::imports_acmd::*;

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("effect_win1", effect_win1, Priority::Default);
	agent.acmd("sound_win1", sound_win1, Priority::Default);
}
