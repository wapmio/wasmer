// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/globals.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;

use crate::runtime::types::Val;
use crate::webassembly::{compile, instantiate, ImportObject, Instance, ResultObject};

use super::_common::{spectest_importobject, NaNCheck};

// Line 4
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (param i32 i32) (result i32)))
      (type (;1;) (func (result i32)))
      (type (;2;) (func (result i64)))
      (type (;3;) (func (param i32)))
      (type (;4;) (func (param i64)))
      (type (;5;) (func (result f32)))
      (type (;6;) (func (result f64)))
      (type (;7;) (func (param f32)))
      (type (;8;) (func (param f64)))
      (type (;9;) (func))
      (type (;10;) (func (param i32) (result i32)))
      (func (;0;) (type 1) (result i32)
        get_global 0)
      (func (;1;) (type 2) (result i64)
        get_global 3)
      (func (;2;) (type 1) (result i32)
        get_global 4)
      (func (;3;) (type 2) (result i64)
        get_global 7)
      (func (;4;) (type 3) (param i32)
        get_local 0
        set_global 4)
      (func (;5;) (type 4) (param i64)
        get_local 0
        set_global 7)
      (func (;6;) (type 5) (result f32)
        get_global 1)
      (func (;7;) (type 6) (result f64)
        get_global 2)
      (func (;8;) (type 5) (result f32)
        get_global 5)
      (func (;9;) (type 6) (result f64)
        get_global 6)
      (func (;10;) (type 7) (param f32)
        get_local 0
        set_global 5)
      (func (;11;) (type 8) (param f64)
        get_local 0
        set_global 6)
      (func (;12;) (type 9))
      (func (;13;) (type 1) (result i32)
        get_global 4
        i32.const 2
        i32.const 3
        select)
      (func (;14;) (type 1) (result i32)
        i32.const 2
        get_global 4
        i32.const 3
        select)
      (func (;15;) (type 1) (result i32)
        i32.const 2
        i32.const 3
        get_global 4
        select)
      (func (;16;) (type 1) (result i32)
        loop (result i32)  ;; label = @1
          get_global 4
          call 12
          call 12
        end)
      (func (;17;) (type 1) (result i32)
        loop (result i32)  ;; label = @1
          call 12
          get_global 4
          call 12
        end)
      (func (;18;) (type 1) (result i32)
        loop (result i32)  ;; label = @1
          call 12
          call 12
          get_global 4
        end)
      (func (;19;) (type 1) (result i32)
        get_global 4
        if (result i32)  ;; label = @1
          call 12
          i32.const 2
        else
          call 12
          i32.const 3
        end)
      (func (;20;) (type 1) (result i32)
        i32.const 1
        if (result i32)  ;; label = @1
          get_global 4
        else
          i32.const 2
        end)
      (func (;21;) (type 1) (result i32)
        i32.const 0
        if (result i32)  ;; label = @1
          i32.const 2
        else
          get_global 4
        end)
      (func (;22;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          get_global 4
          i32.const 2
          br_if 0 (;@1;)
          i32.const 3
          return
        end)
      (func (;23;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          get_global 4
          br_if 0 (;@1;)
          i32.const 3
          return
        end)
      (func (;24;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          get_global 4
          i32.const 2
          br_table 0 (;@1;) 0 (;@1;)
        end)
      (func (;25;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          get_global 4
          br_table 0 (;@1;) 0 (;@1;)
        end)
      (func (;26;) (type 0) (param i32 i32) (result i32)
        get_local 0)
      (func (;27;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          get_global 4
          i32.const 2
          i32.const 0
          call_indirect (type 0)
        end)
      (func (;28;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          get_global 4
          i32.const 0
          call_indirect (type 0)
        end)
      (func (;29;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          i32.const 2
          i32.const 0
          get_global 4
          call_indirect (type 0)
        end)
      (func (;30;) (type 9)
        get_global 4
        i32.const 1
        i32.store)
      (func (;31;) (type 9)
        i32.const 0
        get_global 4
        i32.store)
      (func (;32;) (type 1) (result i32)
        get_global 4
        i32.load)
      (func (;33;) (type 1) (result i32)
        get_global 4
        memory.grow)
      (func (;34;) (type 10) (param i32) (result i32)
        get_local 0)
      (func (;35;) (type 1) (result i32)
        get_global 4
        call 34)
      (func (;36;) (type 1) (result i32)
        get_global 4
        return)
      (func (;37;) (type 9)
        get_global 4
        drop)
      (func (;38;) (type 1) (result i32)
        block (result i32)  ;; label = @1
          get_global 4
          br 0 (;@1;)
        end)
      (func (;39;) (type 10) (param i32) (result i32)
        get_global 4
        set_local 0
        get_local 0)
      (func (;40;) (type 10) (param i32) (result i32)
        get_global 4
        tee_local 0)
      (func (;41;) (type 1) (result i32)
        get_global 4
        set_global 4
        get_global 4)
      (func (;42;) (type 1) (result i32)
        get_global 4
        i32.eqz)
      (func (;43;) (type 1) (result i32)
        get_global 4
        get_global 4
        i32.mul)
      (func (;44;) (type 1) (result i32)
        get_global 0
        i32.const 1
        i32.gt_u)
      (table (;0;) 1 1 anyfunc)
      (memory (;0;) 1)
      (global (;0;) i32 (i32.const -2))
      (global (;1;) f32 (f32.const -0x1.8p+1 (;=-3;)))
      (global (;2;) f64 (f64.const -0x1p+2 (;=-4;)))
      (global (;3;) i64 (i64.const -5))
      (global (;4;) (mut i32) (i32.const -12))
      (global (;5;) (mut f32) (f32.const -0x1.ap+3 (;=-13;)))
      (global (;6;) (mut f64) (f64.const -0x1.cp+3 (;=-14;)))
      (global (;7;) (mut i64) (i64.const -15))
      (export \"get-a\" (func 0))
      (export \"get-b\" (func 1))
      (export \"get-x\" (func 2))
      (export \"get-y\" (func 3))
      (export \"set-x\" (func 4))
      (export \"set-y\" (func 5))
      (export \"get-1\" (func 6))
      (export \"get-2\" (func 7))
      (export \"get-5\" (func 8))
      (export \"get-6\" (func 9))
      (export \"set-5\" (func 10))
      (export \"set-6\" (func 11))
      (export \"as-select-first\" (func 13))
      (export \"as-select-mid\" (func 14))
      (export \"as-select-last\" (func 15))
      (export \"as-loop-first\" (func 16))
      (export \"as-loop-mid\" (func 17))
      (export \"as-loop-last\" (func 18))
      (export \"as-if-condition\" (func 19))
      (export \"as-if-then\" (func 20))
      (export \"as-if-else\" (func 21))
      (export \"as-br_if-first\" (func 22))
      (export \"as-br_if-last\" (func 23))
      (export \"as-br_table-first\" (func 24))
      (export \"as-br_table-last\" (func 25))
      (export \"as-call_indirect-first\" (func 27))
      (export \"as-call_indirect-mid\" (func 28))
      (export \"as-call_indirect-last\" (func 29))
      (export \"as-store-first\" (func 30))
      (export \"as-store-last\" (func 31))
      (export \"as-load-operand\" (func 32))
      (export \"as-memory.grow-value\" (func 33))
      (export \"as-call-value\" (func 35))
      (export \"as-return-value\" (func 36))
      (export \"as-drop-operand\" (func 37))
      (export \"as-br-value\" (func 38))
      (export \"as-set_local-value\" (func 39))
      (export \"as-tee_local-value\" (func 40))
      (export \"as-set_global-value\" (func 41))
      (export \"as-unary-operand\" (func 42))
      (export \"as-binary-operand\" (func 43))
      (export \"as-compare-operand\" (func 44))
      (elem (;0;) (i32.const 0) 26))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 182
fn c1_l182_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c1_l182_action_invoke");
    let result = result_object
        .instance
        .call("c1_l182_action_invoke", &[])
        .expect("Missing result in c1_l182_action_invoke");
    assert_eq!(result, Some(Val::I32(-2 as i32)));
}

// Line 183
fn c2_l183_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c2_l183_action_invoke");
    let result = result_object
        .instance
        .call("c2_l183_action_invoke", &[])
        .expect("Missing result in c2_l183_action_invoke");
    assert_eq!(result, Some(Val::I64(-5 as i64)));
}

// Line 184
fn c3_l184_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c3_l184_action_invoke");
    let result = result_object
        .instance
        .call("c3_l184_action_invoke", &[])
        .expect("Missing result in c3_l184_action_invoke");
    assert_eq!(result, Some(Val::I32(-12 as i32)));
}

// Line 185
fn c4_l185_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c4_l185_action_invoke");
    let result = result_object
        .instance
        .call("c4_l185_action_invoke", &[])
        .expect("Missing result in c4_l185_action_invoke");
    assert_eq!(result, Some(Val::I64(-15 as i64)));
}

// Line 187
fn c5_l187_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c5_l187_action_invoke");
    let result = result_object
        .instance
        .call("c5_l187_action_invoke", &[])
        .expect("Missing result in c5_l187_action_invoke");
    assert_eq!(result, Some(Val::F32((-3.0f32).to_bits())));
}

// Line 188
fn c6_l188_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c6_l188_action_invoke");
    let result = result_object
        .instance
        .call("c6_l188_action_invoke", &[])
        .expect("Missing result in c6_l188_action_invoke");
    assert_eq!(result, Some(Val::F64((-4.0f64).to_bits())));
}

// Line 189
fn c7_l189_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c7_l189_action_invoke");
    let result = result_object
        .instance
        .call("c7_l189_action_invoke", &[])
        .expect("Missing result in c7_l189_action_invoke");
    assert_eq!(result, Some(Val::F32((-13.0f32).to_bits())));
}

// Line 190
fn c8_l190_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c8_l190_action_invoke");
    let result = result_object
        .instance
        .call("c8_l190_action_invoke", &[])
        .expect("Missing result in c8_l190_action_invoke");
    assert_eq!(result, Some(Val::F64((-14.0f64).to_bits())));
}

// Line 192
fn c9_l192_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c9_l192_action_invoke");
    let result = result_object
        .instance
        .call("c9_l192_action_invoke", &[Val::I32(6 as i32)])
        .expect("Missing result in c9_l192_action_invoke");
    assert_eq!(result, None);
}

// Line 193
fn c10_l193_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c10_l193_action_invoke");
    let result = result_object
        .instance
        .call("c10_l193_action_invoke", &[Val::I64(7 as i64)])
        .expect("Missing result in c10_l193_action_invoke");
    assert_eq!(result, None);
}

// Line 194
fn c11_l194_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c11_l194_action_invoke");
    let result = result_object
        .instance
        .call("c11_l194_action_invoke", &[Val::F32((8.0f32).to_bits())])
        .expect("Missing result in c11_l194_action_invoke");
    assert_eq!(result, None);
}

// Line 195
fn c12_l195_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c12_l195_action_invoke");
    let result = result_object
        .instance
        .call("c12_l195_action_invoke", &[Val::F64((9.0f64).to_bits())])
        .expect("Missing result in c12_l195_action_invoke");
    assert_eq!(result, None);
}

// Line 197
fn c13_l197_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c13_l197_action_invoke");
    let result = result_object
        .instance
        .call("c13_l197_action_invoke", &[])
        .expect("Missing result in c13_l197_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 198
fn c14_l198_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c14_l198_action_invoke");
    let result = result_object
        .instance
        .call("c14_l198_action_invoke", &[])
        .expect("Missing result in c14_l198_action_invoke");
    assert_eq!(result, Some(Val::I64(7 as i64)));
}

// Line 199
fn c15_l199_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c15_l199_action_invoke");
    let result = result_object
        .instance
        .call("c15_l199_action_invoke", &[])
        .expect("Missing result in c15_l199_action_invoke");
    assert_eq!(result, Some(Val::F32((8.0f32).to_bits())));
}

// Line 200
fn c16_l200_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c16_l200_action_invoke");
    let result = result_object
        .instance
        .call("c16_l200_action_invoke", &[])
        .expect("Missing result in c16_l200_action_invoke");
    assert_eq!(result, Some(Val::F64((9.0f64).to_bits())));
}

// Line 202
fn c17_l202_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c17_l202_action_invoke");
    let result = result_object
        .instance
        .call("c17_l202_action_invoke", &[])
        .expect("Missing result in c17_l202_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 203
fn c18_l203_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c18_l203_action_invoke");
    let result = result_object
        .instance
        .call("c18_l203_action_invoke", &[])
        .expect("Missing result in c18_l203_action_invoke");
    assert_eq!(result, Some(Val::I32(2 as i32)));
}

// Line 204
fn c19_l204_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c19_l204_action_invoke");
    let result = result_object
        .instance
        .call("c19_l204_action_invoke", &[])
        .expect("Missing result in c19_l204_action_invoke");
    assert_eq!(result, Some(Val::I32(2 as i32)));
}

// Line 206
fn c20_l206_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c20_l206_action_invoke");
    let result = result_object
        .instance
        .call("c20_l206_action_invoke", &[])
        .expect("Missing result in c20_l206_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 207
fn c21_l207_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c21_l207_action_invoke");
    let result = result_object
        .instance
        .call("c21_l207_action_invoke", &[])
        .expect("Missing result in c21_l207_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 208
fn c22_l208_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c22_l208_action_invoke");
    let result = result_object
        .instance
        .call("c22_l208_action_invoke", &[])
        .expect("Missing result in c22_l208_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 210
fn c23_l210_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c23_l210_action_invoke");
    let result = result_object
        .instance
        .call("c23_l210_action_invoke", &[])
        .expect("Missing result in c23_l210_action_invoke");
    assert_eq!(result, Some(Val::I32(2 as i32)));
}

// Line 211
fn c24_l211_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c24_l211_action_invoke");
    let result = result_object
        .instance
        .call("c24_l211_action_invoke", &[])
        .expect("Missing result in c24_l211_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 212
fn c25_l212_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c25_l212_action_invoke");
    let result = result_object
        .instance
        .call("c25_l212_action_invoke", &[])
        .expect("Missing result in c25_l212_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 214
fn c26_l214_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c26_l214_action_invoke");
    let result = result_object
        .instance
        .call("c26_l214_action_invoke", &[])
        .expect("Missing result in c26_l214_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 215
fn c27_l215_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c27_l215_action_invoke");
    let result = result_object
        .instance
        .call("c27_l215_action_invoke", &[])
        .expect("Missing result in c27_l215_action_invoke");
    assert_eq!(result, Some(Val::I32(2 as i32)));
}

// Line 217
fn c28_l217_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c28_l217_action_invoke");
    let result = result_object
        .instance
        .call("c28_l217_action_invoke", &[])
        .expect("Missing result in c28_l217_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 218
fn c29_l218_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c29_l218_action_invoke");
    let result = result_object
        .instance
        .call("c29_l218_action_invoke", &[])
        .expect("Missing result in c29_l218_action_invoke");
    assert_eq!(result, Some(Val::I32(2 as i32)));
}

// Line 220
fn c30_l220_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c30_l220_action_invoke");
    let result = result_object
        .instance
        .call("c30_l220_action_invoke", &[])
        .expect("Missing result in c30_l220_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 221
fn c31_l221_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c31_l221_action_invoke");
    let result = result_object
        .instance
        .call("c31_l221_action_invoke", &[])
        .expect("Missing result in c31_l221_action_invoke");
    assert_eq!(result, Some(Val::I32(2 as i32)));
}

// Line 222
fn c32_l222_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c32_l222_action_invoke");
    let result = result_object
        .instance
        .call("c32_l222_action_invoke", &[])
        .expect("Missing result in c32_l222_action_invoke");
}

#[test]
fn c32_l222_assert_trap() {
    let mut result_object = create_module_1();
    let result = call_protected!(c32_l222_action_invoke(&mut result_object));
    assert!(result.is_err());
}

// Line 224
fn c33_l224_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c33_l224_action_invoke");
    let result = result_object
        .instance
        .call("c33_l224_action_invoke", &[])
        .expect("Missing result in c33_l224_action_invoke");
    assert_eq!(result, None);
}

// Line 225
fn c34_l225_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c34_l225_action_invoke");
    let result = result_object
        .instance
        .call("c34_l225_action_invoke", &[])
        .expect("Missing result in c34_l225_action_invoke");
    assert_eq!(result, None);
}

// Line 226
fn c35_l226_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c35_l226_action_invoke");
    let result = result_object
        .instance
        .call("c35_l226_action_invoke", &[])
        .expect("Missing result in c35_l226_action_invoke");
    assert_eq!(result, Some(Val::I32(1 as i32)));
}

// Line 227
fn c36_l227_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c36_l227_action_invoke");
    let result = result_object
        .instance
        .call("c36_l227_action_invoke", &[])
        .expect("Missing result in c36_l227_action_invoke");
    assert_eq!(result, Some(Val::I32(1 as i32)));
}

// Line 229
fn c37_l229_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c37_l229_action_invoke");
    let result = result_object
        .instance
        .call("c37_l229_action_invoke", &[])
        .expect("Missing result in c37_l229_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 231
fn c38_l231_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c38_l231_action_invoke");
    let result = result_object
        .instance
        .call("c38_l231_action_invoke", &[])
        .expect("Missing result in c38_l231_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 232
fn c39_l232_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c39_l232_action_invoke");
    let result = result_object
        .instance
        .call("c39_l232_action_invoke", &[])
        .expect("Missing result in c39_l232_action_invoke");
    assert_eq!(result, None);
}

// Line 233
fn c40_l233_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c40_l233_action_invoke");
    let result = result_object
        .instance
        .call("c40_l233_action_invoke", &[])
        .expect("Missing result in c40_l233_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 235
fn c41_l235_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c41_l235_action_invoke");
    let result = result_object
        .instance
        .call("c41_l235_action_invoke", &[Val::I32(1 as i32)])
        .expect("Missing result in c41_l235_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 236
fn c42_l236_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c42_l236_action_invoke");
    let result = result_object
        .instance
        .call("c42_l236_action_invoke", &[Val::I32(1 as i32)])
        .expect("Missing result in c42_l236_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 237
fn c43_l237_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c43_l237_action_invoke");
    let result = result_object
        .instance
        .call("c43_l237_action_invoke", &[])
        .expect("Missing result in c43_l237_action_invoke");
    assert_eq!(result, Some(Val::I32(6 as i32)));
}

// Line 239
fn c44_l239_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c44_l239_action_invoke");
    let result = result_object
        .instance
        .call("c44_l239_action_invoke", &[])
        .expect("Missing result in c44_l239_action_invoke");
    assert_eq!(result, Some(Val::I32(0 as i32)));
}

// Line 240
fn c45_l240_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c45_l240_action_invoke");
    let result = result_object
        .instance
        .call("c45_l240_action_invoke", &[])
        .expect("Missing result in c45_l240_action_invoke");
    assert_eq!(result, Some(Val::I32(36 as i32)));
}

// Line 241
fn c46_l241_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c46_l241_action_invoke");
    let result = result_object
        .instance
        .call("c46_l241_action_invoke", &[])
        .expect("Missing result in c46_l241_action_invoke");
    assert_eq!(result, Some(Val::I32(1 as i32)));
}

// Line 244
#[test]
fn c47_l244_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 6, 9, 1, 125, 0, 67, 0, 0, 0,
        0, 11, 10, 8, 1, 6, 0, 65, 1, 36, 0, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 256
#[test]
fn c48_l256_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 6, 10, 1, 125, 0, 67, 0, 0, 0, 0, 140, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 261
#[test]
fn c49_l261_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 6, 1, 125, 0, 32, 0, 11];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 266
#[test]
fn c50_l266_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 6, 10, 1, 125, 0, 67, 0, 0, 128, 63, 140, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 271
#[test]
fn c51_l271_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 7, 1, 127, 0, 65, 0, 1, 11];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 276
#[test]
fn c52_l276_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 5, 1, 127, 0, 1, 11];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 281
#[test]
fn c53_l281_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 6, 9, 1, 127, 0, 67, 0, 0, 0, 0, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 286
#[test]
fn c54_l286_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 6, 8, 1, 127, 0, 65, 0, 65, 0, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 291
#[test]
fn c55_l291_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 6, 11, 2, 127, 0, 65, 0, 11, 126, 0, 35, 1, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 297
#[test]
fn c56_l297_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 4, 1, 127, 0, 11];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 302
#[test]
fn c57_l302_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 6, 6, 1, 127, 0, 35, 0, 11];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 307
#[test]
fn c58_l307_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 6, 11, 2, 127, 0, 35, 1, 11, 127, 0, 65, 0, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 311

#[test]
fn test_module_1() {
    let mut result_object = create_module_1();
    // We group the calls together
    start_module_1(&mut result_object);
    c1_l182_action_invoke(&mut result_object);
    c2_l183_action_invoke(&mut result_object);
    c3_l184_action_invoke(&mut result_object);
    c4_l185_action_invoke(&mut result_object);
    c5_l187_action_invoke(&mut result_object);
    c6_l188_action_invoke(&mut result_object);
    c7_l189_action_invoke(&mut result_object);
    c8_l190_action_invoke(&mut result_object);
    c9_l192_action_invoke(&mut result_object);
    c10_l193_action_invoke(&mut result_object);
    c11_l194_action_invoke(&mut result_object);
    c12_l195_action_invoke(&mut result_object);
    c13_l197_action_invoke(&mut result_object);
    c14_l198_action_invoke(&mut result_object);
    c15_l199_action_invoke(&mut result_object);
    c16_l200_action_invoke(&mut result_object);
    c17_l202_action_invoke(&mut result_object);
    c18_l203_action_invoke(&mut result_object);
    c19_l204_action_invoke(&mut result_object);
    c20_l206_action_invoke(&mut result_object);
    c21_l207_action_invoke(&mut result_object);
    c22_l208_action_invoke(&mut result_object);
    c23_l210_action_invoke(&mut result_object);
    c24_l211_action_invoke(&mut result_object);
    c25_l212_action_invoke(&mut result_object);
    c26_l214_action_invoke(&mut result_object);
    c27_l215_action_invoke(&mut result_object);
    c28_l217_action_invoke(&mut result_object);
    c29_l218_action_invoke(&mut result_object);
    c30_l220_action_invoke(&mut result_object);
    c31_l221_action_invoke(&mut result_object);
    c33_l224_action_invoke(&mut result_object);
    c34_l225_action_invoke(&mut result_object);
    c35_l226_action_invoke(&mut result_object);
    c36_l227_action_invoke(&mut result_object);
    c37_l229_action_invoke(&mut result_object);
    c38_l231_action_invoke(&mut result_object);
    c39_l232_action_invoke(&mut result_object);
    c40_l233_action_invoke(&mut result_object);
    c41_l235_action_invoke(&mut result_object);
    c42_l236_action_invoke(&mut result_object);
    c43_l237_action_invoke(&mut result_object);
    c44_l239_action_invoke(&mut result_object);
    c45_l240_action_invoke(&mut result_object);
    c46_l241_action_invoke(&mut result_object);
}
fn create_module_2() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (import \"spectest\" \"global_i32\" (global (;0;) i32))
      (func (;0;) (type 0) (result i32)
        get_global 0)
      (func (;1;) (type 0) (result i32)
        get_global 1)
      (global (;1;) i32 (get_global 0))
      (export \"get-0\" (func 0))
      (export \"get-0-ref\" (func 1)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_2(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 318
fn c60_l318_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c60_l318_action_invoke");
    let result = result_object
        .instance
        .call("c60_l318_action_invoke", &[])
        .expect("Missing result in c60_l318_action_invoke");
    assert_eq!(result, Some(Val::I32(666 as i32)));
}

// Line 319
fn c61_l319_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c61_l319_action_invoke");
    let result = result_object
        .instance
        .call("c61_l319_action_invoke", &[])
        .expect("Missing result in c61_l319_action_invoke");
    assert_eq!(result, Some(Val::I32(666 as i32)));
}

// Line 322
#[test]
fn c62_l322_assert_malformed() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 2, 148, 128, 128, 128, 0, 1, 8, 115, 112, 101, 99, 116, 101,
        115, 116, 10, 103, 108, 111, 98, 97, 108, 95, 105, 51, 50, 3, 127, 2,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is malformed"
    );
}

// Line 335
#[test]
fn c63_l335_assert_malformed() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 2, 148, 128, 128, 128, 0, 1, 8, 115, 112, 101, 99, 116, 101,
        115, 116, 10, 103, 108, 111, 98, 97, 108, 95, 105, 51, 50, 3, 127, 255,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is malformed"
    );
}

// Line 348

#[test]
fn test_module_2() {
    let mut result_object = create_module_2();
    // We group the calls together
    start_module_2(&mut result_object);
    c60_l318_action_invoke(&mut result_object);
    c61_l319_action_invoke(&mut result_object);
}
fn create_module_3() -> ResultObject {
    let module_str = "(module
      (global (;0;) i32 (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_3(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 352
#[test]
fn c65_l352_assert_malformed() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 6, 134, 128, 128, 128, 0, 1, 127, 2, 65, 0, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is malformed"
    );
}

// Line 364
#[test]
fn c66_l364_assert_malformed() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 6, 134, 128, 128, 128, 0, 1, 127, 255, 65, 0, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is malformed"
    );
}

#[test]
fn test_module_3() {
    let mut result_object = create_module_3();
    // We group the calls together
    start_module_3(&mut result_object);
}
