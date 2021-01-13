// c2rust.go

use std::collections::HashMap;

pub fn init() {
	let mut variable = HashMap::new();
	variable.insert("int", "i32");
	variable.insert("float", "f32");
	variable.insert("double", "f64");
}

pub struct Var{
	label:       String,
	vartype:     String,
	initializer: String,
	mutable:     bool,
	used:        bool
}

pub struct Env{
	parent: *mut Env,
	vars:   HashMap::new()
}

pub fn NewEnv(parent: *mut Env) -> *mut Env {
	return &Env{
		parent: parent,
		vars:   HashMap::new(),
	}
}

impl Env{
	fn  NewVar(&self, label: String, vartype: String, initializer: String) {
		if vt, ok := vartable[vartype]; ok {
			let v = &Var{
				label:       label,
				vartype:     vt,
				initializer: initializer,
				mutable:     false,
				used:        false,
			};
			self.vars[label] = v;
		} else {
			println!("error");
		}
	}
	
	fn GetVar(&self, varlabel: String) -> (*mut Var, bool) {
		let current = self;
		while current != None {
			if v, ok := self.vars[varlabel]; ok {
				return v, true;
			} else {
				current = self.parent;
			}
		}
		return None, false;
	}
}


impl CFunction{
	fn toRust(&self) -> *mut RFunction {
		let mut env = NewEnv(None);
		// args
		let mut args = Vec<RVar>;
		for _, x := range f.args {
			v := RVar{
				label:   x.label,
				vartype: vartable[x.vartype],
			}
			args = append(args, v)
		}
		for _, s := range f.body {
			env = s.UpdateEnv(env)
		}
		body := []RWriter{}
		// decl block
		for k, v := range env.vars {
			fmt.Println(v.vartype)
			rv := RVar{
				label:   k,
				vartype: v.vartype,
				mutable: v.mutable,
			}
			s := &RVarDecl{
				rvar:        rv,
				initializer: v.initializer,
			}
			body = append(body, s)
		}
		// TODO: statements
		return &RFunction{
			label:      f.label,
			args:       args,
			body:       body,
			returntype: vartable[f.returntype],
		};
	}
}

impl CVarDecl{
	fn UpdateEnv(&self, env: *mut Env) -> *mut Env {
		env.NewVar(self.cvar.label, self.cvar.vartype, self.initializer);
		return env;
	}
}

impl CAssignment{
	fn UpdateEnv(&self, env: *mut Env) -> *mut Env {
		v, _ := env.GetVar(self.cvar.label)
		// the right value can be evaluated?
		if !v.used {
			v.initializer = self.right
			v.used = true
		} else {
			v.mutable = true
		}
		return env
	}
}