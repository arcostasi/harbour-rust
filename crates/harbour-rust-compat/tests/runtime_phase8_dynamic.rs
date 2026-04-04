use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeContext, RuntimeError, Value, empty, eval, valtype};

fn result_text(result: Result<Value, RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

fn dynamic_runtime_baseline() -> String {
    let mut context = RuntimeContext::new();
    context.define_public("counter", Value::from(10_i64));
    context.define_public("total", Value::from(10_i64));
    context.push_private_frame();
    context.define_private("counter", Value::from(4_i64));

    let add_block = Value::codeblock("{|x, y| x + y }", |arguments, _context| {
        let left = arguments.first().cloned().unwrap_or(Value::Nil);
        let right = arguments.get(1).cloned().unwrap_or(Value::Nil);
        left.add(&right)
    });
    let memvar_block = Value::codeblock("{|| counter + 1 }", |_, context| {
        context.read_memvar("counter").add(&Value::from(1_i64))
    });

    let mut out = String::new();
    out.push_str(&format!(
        "Read PRIVATE counter => {}\n",
        context.read_memvar("counter").to_output_string()
    ));
    out.push_str(&format!(
        "Read PUBLIC total => {}\n",
        context.read_memvar("total").to_output_string()
    ));
    out.push_str(&format!(
        "Eval(add, 2, 3) => {}\n",
        result_text(eval(
            Some(&add_block),
            &[Value::from(2_i64), Value::from(3_i64)],
            &mut context
        ))
    ));
    out.push_str(&format!(
        "Eval(memvar) => {}\n",
        result_text(eval(Some(&memvar_block), &[], &mut context))
    ));
    out.push_str(&format!(
        "ValType(add) => {}\n",
        result_text(valtype(Some(&add_block)))
    ));
    out.push_str(&format!(
        "Empty(add) => {}\n",
        result_text(empty(Some(&add_block)))
    ));
    out.push_str(&format!(
        "Assign PRIVATE counter => {}\n",
        context
            .assign_memvar("counter", Value::from(5_i64))
            .to_output_string()
    ));
    out.push_str(&format!(
        "Read PRIVATE counter after assign => {}\n",
        context.read_memvar("counter").to_output_string()
    ));
    context.pop_private_frame();
    out.push_str(&format!(
        "Read counter after pop => {}\n",
        context.read_memvar("counter").to_output_string()
    ));
    out.push_str(&format!(
        "Eval(NIL) => {}\n",
        result_text(eval(Some(&Value::Nil), &[], &mut context))
    ));
    out
}

#[test]
fn phase8_dynamic_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/phase8_dynamic_runtime.prg",
    ))
    .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn phase8_dynamic_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_memvar) =
        read_upstream_or_skip("harbour-core/tests/memvar.prg", "upstream memvar test")
    else {
        return;
    };
    let Some(upstream_codeblock_doc) =
        read_upstream_or_skip("harbour-core/doc/codebloc.txt", "upstream codeblock doc")
    else {
        return;
    };
    let Some(upstream_codeblock_vm) =
        read_upstream_or_skip("harbour-core/src/vm/codebloc.c", "upstream codeblock vm")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/phase8_dynamic_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_memvar.contains("PUBLIC overrided by PRIVATE"));
    assert!(upstream_memvar.contains("PRIVATE privVar := \" (PRIVATE in MAIN) \""));
    assert!(
        upstream_codeblock_doc
            .contains("Parameters passed to a codeblock are placed on the eval stack before a")
    );
    assert!(upstream_codeblock_doc.contains(
        "there is no safe method to find if a codeblock will be accessed from an outside"
    ));
    assert!(upstream_codeblock_vm.contains("hb_codeblockNew"));

    assert_eq!(dynamic_runtime_baseline(), expected);
}
