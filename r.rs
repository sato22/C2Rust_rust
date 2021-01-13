// r.go

pub struct RVar {
	label:   String,
	vartype: String,
	mutable: bool
}

pub struct RVarDecl {
	rvar:        RVar,
	initializer: String
}

pub struct RAssignment {
	rvar:  RVar,
	right: String
}

pub struct RFunction {
	label:      String,
	args:       Vec<RVar>,
	body:       Vec<RWriter>,
	returntype: String
}

trait RWriter {
	fn write(&self);
}

impl RWriter for RFunction {

    fn write(f: *const RFunction) {
    	let mut argstr = Vec::new();
    	for x in 0..f.args.len(){
    		if x.mutable {
    			argstr.push(format!("mut {}: {}", x.label, x.vartype));
    		} else {
    			argstr.push(format!("{}: {}", x.label, x.vartype));
    		}
    	}
    	if f.returntype != "" {
    		print!("fn {}({}) -> {} {{\n", f.label, format!("{}{}",argstr, ", "), f.returntype)
    	} else {
    		print!("fn {}({}) {{\n", f.label, format!("{}{}",argstr, ", "))
    	}
    	for x in 0..f.body.len() {
    		x.write()
    	}
    	println!("}}");
    }
}

impl RWriter for RVarDecl {
    fn write(d: *const RVarDecl) {
    	if d.rvar.mutable {
    		print!("let mut {}: {}", d.rvar.label, d.rvar.vartype);
    	} else {
    		print!("let {}: {}", d.rvar.label, d.rvar.vartype);
    	}
    	if d.initializer != "" {
    		print!(" = {}", d.initializer);
    	}
    	println!("");
    }
}
