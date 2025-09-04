mod catch;
mod throwf;
//mod throwb;
//mod throwhi;
mod throwlw;

pub fn install(agent: &mut smashline::Agent) {
    catch::install(agent);
    throwf::install(agent);
    //throwb::install(agent);
    //throwhi::install(agent);
    throwlw::install(agent);
}
