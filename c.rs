// c.go
pub struct CVar {
	label:   String,
	vartype: String
}

pub struct CVarDecl {
	cvar:        CVar,
	initializer: String // this should be an expr?
}

pub struct CAssignment {
	cvar:  CVar,
	right: String // this should be an expr?
}

pub struct CFunction {
	label:      String,
	args:       Vec<CVar>,
	body:       Vec<CStatement>,
	returntype: String
}

impl CStatement{
	UpdateEnv(*Env) *Env
}