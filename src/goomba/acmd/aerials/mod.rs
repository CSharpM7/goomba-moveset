mod attackairn;
mod attackairf;
//mod attackairb;
//mod attackairhi;
//mod attackairlw;

pub fn install(agent: &mut smashline::Agent) {
    attackairn::install(agent);
    attackairf::install(agent);
    //attackairb::install(agent);
    //attackairhi::install(agent);
    //attackairlw::install(agent);
}
