mod attacks4;

//mod specialn;
mod specials;
//mod specialhi;
//mod speciallw;

pub fn install(agent: &mut smashline::Agent) {
    attacks4::install(agent);

	//specialn::install(agent);
	specials::install(agent);
	//specialhi::install(agent);
    //speciallw::install(agent);
}