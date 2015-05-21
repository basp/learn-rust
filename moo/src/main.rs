#[derive(Debug)]
enum Error {
	NONE,
	TYPE,
	DIV,
	PERM,
	PROPNF,
	// etc.
}

#[derive(Debug)]
enum Val {
	Int(i32),
	Float(f64),
	Str(String),
	List(Vec<Val>),
	Error(Error),
	Obj(i32)
}

fn main() {
	let v1 = Val::Int(123);
	
	let v2 = Val::Str("foo".to_string());
	
	let v3 = Val::List(vec![
		Val::Int(1), 
		Val::Str("foo".to_string()), 
		Val::Int(3)
	]);

	let v4 = Val::Float(1.23);

	let v5 = Val::Obj(456);

	let v6 = Val::Error(Error::NONE);

	let vals = vec![v1, v2, v3, v4, v5, v6];

	for v in vals {
		println!("{:?}", v);
	}
}
