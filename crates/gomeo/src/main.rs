use inkwell::{context::Context, module::Linkage, values::BasicMetadataValueEnum, AddressSpace};

mod ast;
mod codegen;
mod lexer;
mod parser;

fn main() {
    let context = Context::create();
    let module = context.create_module("gomeo");
    let builder = context.create_builder();

    let i32_type = context.i32_type();
    let i8_ptr_type = context.ptr_type(AddressSpace::default());

    let puts_type = i32_type.fn_type(&[i8_ptr_type.into()], false);

    let puts = module.add_function("puts", puts_type, Some(Linkage::External));

    // fun main() -> i32

    let main_type = i32_type.fn_type(&[], false);
    let main_fn = module.add_function("main", main_type, None);

    let entry = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(entry);

    let text = builder
        .build_global_string_ptr("Hello world", "hello")
        .expect("[CRITICAL] The text could not be created");

    builder
        .build_call(
            puts,
            &[BasicMetadataValueEnum::from(text.as_pointer_value())],
            "puts_call",
        )
        .expect("[CRITICAL] Failed to generate puts function");

    builder
        .build_return(Some(&i32_type.const_int(0, false)))
        .expect("[CRITICAL] Failed to generate return 0;");

    println!("{}", module.print_to_string().to_string());

    module
        .print_to_file("hello.ll")
        .expect("[CRITICAL] Failed to generate code");
}
