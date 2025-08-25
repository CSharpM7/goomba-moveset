pub mod imports_acmd {
    pub use {
        crate::vars::*,
        crate::singleslot::*,
        crate::goomba::{*,articles::*},
        
        sharpsmashlinesuite::{
            ext::*,
            util::{self, *},
            *,
        },

        smash::{
            app::{
                self,
                lua_bind::*,
                sv_animcmd::{frame, wait},
                *,
            },
            hash40,
            lib::lua_const::*,
            lua2cpp::*,
            phx::*,
        },
        smash_script::{macros::*, *},
        smashline::*,
    };
    pub unsafe extern "C" fn empty_acmd(agent: &mut L2CAgentBase) {}
    pub unsafe fn common_effect_color(agent: &mut L2CAgentBase) {
        let r = 2.8; let g = 0.5; let b = 0.1;
        macros::LAST_PARTICLE_SET_COLOR(agent, r,g,b);
        LAST_EFFECT_SET_COLOR(agent, r,g,b);
    }
}

pub mod imports_agent {
    pub use {
        crate::vars::*,
        crate::singleslot::*,
        crate::goomba::{*,articles::*},

        sharpsmashlinesuite::{
            ext::*,
            util::{self, *},
            *,
        },

        smash::{
            app::{self, lua_bind::*, *},
            hash40,
            lib::{lua_const::*, L2CAgent, L2CValue},
            lua2cpp::*,
            phx::*,
        },
        smash_script::{macros::*, *},
        smashline::{Main, *},
    };
    pub unsafe extern "C" fn empty_status(agent: &mut L2CAgentBase) -> L2CValue {
        0.into()
    }
}
pub mod imports_status {
    pub use {
        crate::imports::imports_agent::*,
        smashline::{End, Exec, Init, Main, Pre, *},
    };
}
