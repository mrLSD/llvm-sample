use llvm_sys::core::*;
use llvm_sys::target::*;

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
		
		LLVMDumpModule(module);

		LLVMDisposeBuilder(builder);
		LLVMDisposeModule(module);
		LLVMDoubleTypeInContext(ctx);
	};
}
/*
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