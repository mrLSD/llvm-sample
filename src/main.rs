use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::targets::{TargetMachine, Target, InitializationConfig, RelocMode, CodeModel, FileType};
use inkwell::OptimizationLevel;
use inkwell::module::Module;
use std::path::Path;
use std::process::{Command};

fn apply_target_to_module(target_machine: &TargetMachine, module: &Module) {
    module.set_triple(&target_machine.get_triple());
    module.set_data_layout(&target_machine.get_target_data().get_data_layout());
}

fn get_native_target_machine() -> TargetMachine {
    Target::initialize_native(&InitializationConfig::default())
        .expect("Failed to initialize native target");
    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).unwrap();
    target
        .create_target_machine(
            &target_triple,
            &TargetMachine::get_host_cpu_name().to_string(),
            &TargetMachine::get_host_cpu_features().to_string(),
            OptimizationLevel::Aggressive,
            RelocMode::PIC,
            CodeModel::Medium,
        )
        .unwrap()
}

static LLVM_IR_SRC: &str = r##"; ModuleID = '/home/pc/dev/rs/cintegration/res.ll'
source_filename = "main.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@.str = private unnamed_addr constant [8 x i8] c"# Test\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
  %1 = alloca i32, align 4
  store i32 0, i32* %1, align 4
  %2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @.str, i64 0, i64 0))
  ret i32 0
}

declare dso_local i32 @printf(i8*, ...) #1

attributes #0 = { noinline nounwind optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{!"clang version 11.0.0"}"##; 

fn main() {
	let context = Context::create();
	let str = LLVM_IR_SRC.to_string();
	let memory_buffer = MemoryBuffer::create_from_memory_range(str.as_bytes(), "main");
	let module = context.create_module_from_ir(memory_buffer).unwrap();
	
	let target_machine = get_native_target_machine();
	apply_target_to_module(&target_machine, &module);

	std::fs::create_dir("bin");
	target_machine
		.write_to_file(&module, FileType::Object, Path::new("bin/app.o"))
		.unwrap();
	Command::new("ar")
		.args(&["crs", "bin/libap.a", "bin/app.o"])
		.spawn()
		.expect("Expect success")
		.wait();
/*
	Command::new("gcc")
		.args(&[ 
			"-o", "ap",
			"libap.a"])
		.spawn()
		.expect("Expect success");
*/
	
	Command::new("ld")
		.args(&[ 
			"-o", "bin/ap",
			"-dynamic-linker",
			"/lib64/ld-linux-x86-64.so.2",
			"/usr/lib/x86_64-linux-gnu/crt1.o",
			"/usr/lib/x86_64-linux-gnu/crti.o",
			"/usr/lib/x86_64-linux-gnu/crtn.o",
			"-lc",
			"bin/libap.a"])
		.spawn()
		.expect("Expect success")
		.wait();
	std::fs::remove_file(Path::new("bin/app.o"));
}

/*use llvm_sys::core::*;
use llvm_sys::target::*;
use llvm_sys::analysis::LLVMVerifyFunction;
use llvm_sys::analysis::LLVMVerifierFailureAction::LLVMAbortProcessAction;

fn main() {
	unsafe {
		LLVM_InitializeNativeTarget();
		LLVM_InitializeNativeAsmPrinter();
		
		// Init basic
		let ctx = LLVMContextCreate();
		let module_name = "main_module\0".as_ptr() as *const _;
		let module = LLVMModuleCreateWithNameInContext(module_name, ctx);
		let builder = LLVMCreateBuilderInContext(ctx);
		
		// Create function
		let i32_ty = LLVMInt32TypeInContext(ctx);
		let mut fn_params = [i32_ty];
		let fn_ty = LLVMFunctionType(i32_ty, fn_params.as_mut_ptr(), fn_params.len() as u32, 0);
		let fn_name = "main\0".as_ptr() as *const _; 
		let main_fn = LLVMAddFunction(module, fn_name, fn_ty);
		
		// Create basic block and add it to function
		let fn_block_name = "entry\0".as_ptr() as *const _;
		//let fn_block_name = CString::new("entry").unwrap();
		let fn_block = LLVMAppendBasicBlockInContext(ctx, main_fn, fn_block_name);
		LLVMPositionBuilderAtEnd(builder, fn_block);
		
		// Body of function
		let val1 = LLVMGetParam(main_fn, 0);
		let val2 = LLVMConstInt(i32_ty, 20, 0);
		
		let tmp_name = "addtmp\0".as_ptr() as *const _;
		let res = LLVMBuildAdd(builder, val1, val2, tmp_name);
		
		LLVMBuildRet(builder, res);
		LLVMVerifyFunction(main_fn, LLVMAbortProcessAction);
		
		LLVMDumpModule(module);

		LLVMDisposeBuilder(builder);
		LLVMDisposeModule(module);
		LLVMDoubleTypeInContext(ctx);
	};
}

use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;
use std::path::Path;

type AppFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;
type PutChar = unsafe extern "C" fn(u64) -> u64;

fn main() {
    let context = Context::create();
    let module = context.create_module("app");
    let builder = context.create_builder();
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
    let function = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");

    builder.position_at_end(basic_block);

    let x = function.get_nth_param(0).unwrap().into_int_value();
    let y = function.get_nth_param(1).unwrap().into_int_value();
    let z = function.get_nth_param(2).unwrap().into_int_value();

    let sum = builder.build_int_add(x, y, "sum");
    let sum = builder.build_int_add(sum, z, "sum");

    builder.build_return(Some(&sum));
    // module.verify().unwrap();

    let path = Path::new("module.bc");
    module.write_bitcode_to_path(&path);
    module.print_to_file("module");
    
    let app: JitFunction<AppFunc> = unsafe { execution_engine.get_function("main").ok() }.unwrap();
    let res = unsafe { app.call(10, 20, 30) };
    println!("{:?}", res)
}
*/