#![doc(hidden)]
//! This file was automatically generated by a build script. Please don't edit
//! me!

use hematita::{
	ast::{lexer::Lexer, parser::{TokenIterator, parse_block}},
	compiler::compile_block, lua_lib::standard_globals,
	vm::{value::{IntoNillable, Table, Value}, VirtualMachine},
	lua_tuple
};
use itertools::Itertools;
use std::{process::Command, sync::{Arc, Mutex}};
use diff::{Result as Diff, lines};

fn table_to_vector<'n>(table: &Table<'n>) -> Vec<Option<Value<'n>>> {
	let table = table.data.lock().unwrap();
	let end = table.get(&Value::Integer(0)).unwrap().integer().unwrap();

	(1..=end)
		.map(|index| table.get(&Value::Integer(index)).cloned())
		.collect()
}

/// Executes [code] using Hematita.
fn hematita(code: &str) -> String {
	let output = Arc::new(Mutex::new(String::new()));
	let print_output = output.clone();
	let print = move |arguments: Arc<_>, _: &'_ _| {
		let message = table_to_vector(&*arguments).into_iter()
			.map(|argument| format!("{}", argument.nillable()))
			.join("	");
		let mut output = output.lock().unwrap();
		output.push_str(&message); output.push('\n');
		Ok(lua_tuple![].arc())
	};

	let tokens = Lexer {source: code.chars().peekable()}.peekable();
	let parsed = parse_block(&mut TokenIterator(tokens)).unwrap();
	let function = compile_block(&parsed).into();

	let globals = {
		let globals = standard_globals();
		{
			let mut data = globals.data.lock().unwrap();
			data.insert(Value::new_string("print"), Value::NativeFunction(&print));
		}
		globals
	};

	let arguments = lua_tuple![].arc();
	let virtual_machine = VirtualMachine::new(globals);
	virtual_machine.execute(&function, arguments).unwrap();

	let result = print_output.lock().unwrap().clone();
	result
}

/// Executes [code] using the standard Lua interpreter.
fn standard(code: &str) -> String {
	let stdout = Command::new("lua")
		.args(["-e", code])
		.output()
		.unwrap()
		.stdout;
	String::from_utf8_lossy(&stdout).into_owned()
}

#[test]
fn fun_quine() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/funQuine.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "funQuine.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn library_pcall() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/libraryPcall.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "libraryPcall.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn operator_call() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/operatorCall.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "operatorCall.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn operator_length() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/operatorLength.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "operatorLength.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn operator_logical_and() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/operatorLogicalAnd.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "operatorLogicalAnd.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn operator_logical_or() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/operatorLogicalOr.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "operatorLogicalOr.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn statement_function() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/statementFunction.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "statementFunction.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn statement_generic_for() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/statementGenericFor.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "statementGenericFor.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn statement_local() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/statementLocal.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "statementLocal.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn statement_method() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/statementMethod.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "statementMethod.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn statement_numeric_for() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/statementNumericFor.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "statementNumericFor.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn statement_while_repeat() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/statementWhileRepeat.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "statementWhileRepeat.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn up_value_deep() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/upValueDeep.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "upValueDeep.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}

#[test]
fn up_value_modify() -> Result<(), ()> {
	static CODE: &str = include_str!("/pmnt/data/danii/Programming/Rust/luamoon/hematita/tests/matches_standard/lua/upValueModify.lua");

	let hematita = hematita(CODE);
	let standard = standard(CODE);

	if hematita != standard {
		eprintln!("The output of {:?} from Hematita differs from the standard Lua implementation.", "upValueModify.lua");
		lines(&hematita, &standard).into_iter()
			.for_each(|result| match result {
				Diff::Left(left) => eprintln!("\x1B[31m-{}\x1B[0m", left),
				Diff::Right(right) => eprintln!("\x1B[32m+{}\x1B[0m", right),
				Diff::Both(both, _) => eprintln!(" {}", both)
			});
		Err(())
	} else {Ok(())}
}