use std::path::Path;

use inkwell::{AddressSpace, context::Context, module::Linkage, values::BasicMetadataValueEnum};

pub fn generate_code(save_path: &Path) -> Result<(), String> {
    let context = Context::create();
    let module = context.create_module("gomeo");
    let builder = context.create_builder();

    let i32_type = context.i32_type();
    let i8_ptr_type = context.ptr_type(AddressSpace::default());

    let puts = i32_type.fn_type(&[i8_ptr_type.into()], false);
    let puts_function = module.add_function("puts", puts, Some(Linkage::External));

    let main_type = i32_type.fn_type(&[], false);
    let main_fn = module.add_function("main", main_type, None);

    let entry = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(entry);

    let text = builder
        .build_global_string_ptr("Hello world", "hello")
        .unwrap(); // To be replaced

    if let Err(e) = builder.build_call(
        puts_function,
        &[BasicMetadataValueEnum::from(text.as_pointer_value())],
        "puts_call",
    ) {
        return Err(format!("Failed to generate call: {}", e));
    }

    let ret = i32_type.const_int(0, false);
    if let Err(e) = builder.build_return(Some(&ret)) {
        return Err(format!("Failed to generate return: {}", e));
    }

    if let Err(e) = module.print_to_file(save_path) {
        return Err(format!("Failed to generate file: {}", e));
    }
    Ok(())
}
