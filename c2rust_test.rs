// c2rust_test.go

use c::*;
use r::*;
use c2rust::*;


#[cfg(test)]
mod tests {
	#[test]
	fn TestVar() {
		let mut env = NewEnv(nil);         // enter a scope
		env.NewVar("a", "int", ""); // int a;
		println!("{:?}",env);
		// a = 1
		val := "1" // literal 1
		if x, ok := env.GetVar("a"); ok {
			x.initializer = val
			fmt.Println(x)
		}
		fmt.Println(env)
		// ....
		if x, ok := env.GetVar("a"); ok {
			x.mutable = false
			fmt.Println(x)
		}
		println!("{}",env);
	}


	#[test]     // cargo test
	fn TestC() {
		/*
		   int f(int a, int b) {
		       int c;
		       c = 10;
		       // ....
		   }
		*/
		let a = CVar{
			label:   "a",
			vartype: "int",
		};
		let b = CVar{
			label:   "b",
			vartype: "int",
		};
		let s1 = CVarDecl{
			cvar: CVar{
				label:   "c",
				vartype: "int",
			},
			initializer: "",
		};
		let s2 = CAssignment{
			cvar: CVar{
				label:   "c",
				vartype: "int",
			},
			right: "10",
		};
	
		let f = CFunction{
			label:      "f",
			/*
			args:       []CVar{a, b},
			body:       []CStatement{&s1, &s2},
			の書き換え
			*/
			args:       vec!{CVar{a, b}},
			body:       vec!{CStatement{&s1, &s2}},
			returntype: "int",
		};
	
		println!("{}",f);
	}
	
	#[test]
	fn TestRust() {
		let a = RVar{
			label:   "a",
			vartype: "i32",
		};
		let b = RVar{
			label:   "b",
			vartype: "i32",
		};
		let s1 = RVarDecl{
			rvar: RVar{
				label:   "c",
				vartype: "i32",
			},
			initializer: "1",
		};
		let f = RFunction{
			label:      "f",
			args:       vec!{RVar{a, b}},
			body:       vec!{RWriter{&s1}},
			returntype: "i32",
		};
	
		f.write();
	}
	
	#[test]
	fn TestC2R() {
		/*
		   int f(int a, int b) {
		       int c;
		       c = 10;
		       // ....
		   }
		*/
		let a = CVar{
			label:   "a",
			vartype: "int",
		};
		let b = CVar{
			label:   "b",
			vartype: "int",
		};
		let s1 = CVarDecl{
			cvar: CVar{
				label:   "c",
				vartype: "int",
			},
			initializer: "",
		};
		let s2 = CAssignment{
			cvar: CVar{
				label:   "c",
				vartype: "int",
			},
			right: "10",
		};
	
		let f = CFunction{
			label:      "f",
			args:       vec!{CVar{a, b}},
			body:       vec!{CStatement{&s1, &s2}},
			returntype: "int",
		};
	
		println!("{}",f);
	
		let rf = f.toRust();
		print!("convert c to rust");
		rf.write();
	}
	
	#[test]
	fn TestC2R2() {
		/*
		   int f(int a, int b) {
		       int c;
		       c = 10;
		       c = 20;
		       // ....
		   }
		*/
		let a = CVar{
			label:   "a",
			vartype: "int",
		};
		let b = CVar{
			label:   "b",
			vartype: "int",
		};
		let s1 = CVarDecl{
			cvar: CVar{
				label:   "c",
				vartype: "int",
			},
			initializer: "",
		};
		let s2 = CAssignment{
			cvar: CVar{
				label:   "c",
				vartype: "int",
			},
			right: "10",
		};
		let s3 = CAssignment{
			cvar: CVar{
				label:   "c",
				vartype: "int",
			},
			right: "20",
		};
	
		let f = CFunction{
			label:      "f",
			args:       vec!{CVar{a, b}},
			body:       vec!{CStatement{&s1, &s2, &s3}},
			returntype: "int",
		};
	
		println!("{}",f);
	
		let rf = f.toRust();
		println!("convert c to rust");
		rf.write();
	}
}