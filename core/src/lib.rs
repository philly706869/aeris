pub mod acm;
pub mod ui;

#[cfg(test)]
mod example {
    use inkwell::{AddressSpace, IntPredicate, context::Context, module::Linkage};

    #[test]
    #[ignore]
    fn llvm_factorial_example() {
        // #include <stdio.h>
        //
        // int fact(int n) {
        //     int result = 1;
        //     while (n > 1) {
        //         result = result * n;
        //         n = n - 1;
        //     }
        //     return result;
        // }
        //
        // int main() {
        //     int x = 5;
        //     int y = fact(x);
        //     printf("fact(%d) = %d\n", x, y);
        //     return 0;
        // }

        let context = Context::create();
        let module = context.create_module("");
        let builder = context.create_builder();

        // Type Definition
        let i32_type = context.i32_type();
        let ptr_type = context.ptr_type(AddressSpace::default());

        // Global Definition
        let printf_format_string = {
            let text = "fact(%d) = %d\n";
            let const_string = context.const_string(text.as_bytes(), true);
            let global = module.add_global(const_string.get_type(), None, "");
            global.set_initializer(&const_string);
            global.set_constant(true);
            global.as_pointer_value()
        };

        let printf_fn = {
            // Printf Function
            let name = "printf";
            let fn_type = i32_type.fn_type(&[ptr_type.into()], true);
            let linkage = None;
            let function = module.add_function(name, fn_type, linkage);

            function
        };

        let factorial_fn = {
            // Factorial Function Definition
            let name = "factorial";
            let fn_type = i32_type.fn_type(&[i32_type.into()], false);
            let linkage = Some(Linkage::Internal);
            let function = module.add_function(name, fn_type, linkage);

            let entry_block = context.append_basic_block(function, "entry");
            let loop_block = context.append_basic_block(function, "loop");
            let then_block = context.append_basic_block(function, "then");
            let return_block = context.append_basic_block(function, "return");

            // entry:
            builder.position_at_end(entry_block);
            let n = {
                let value = function.get_first_param().unwrap();
                let ptr = builder.build_alloca(value.get_type(), "n").unwrap();
                builder.build_store(ptr, value).unwrap();
                ptr
            };
            let result = {
                let value = i32_type.const_int(1, false);
                let ptr = builder.build_alloca(value.get_type(), "result").unwrap();
                builder.build_store(ptr, value).unwrap();
                ptr
            };
            builder.build_unconditional_branch(loop_block).unwrap();

            // loop:
            builder.position_at_end(loop_block);
            let condition = {
                let op = IntPredicate::SGT;
                let lhs = builder
                    .build_load(i32_type, n, "")
                    .unwrap()
                    .into_int_value();
                let rhs = i32_type.const_int(1, false);
                builder.build_int_compare(op, lhs, rhs, "").unwrap()
            };
            builder
                .build_conditional_branch(condition, then_block, return_block)
                .unwrap();

            // then:
            builder.position_at_end(then_block);
            {
                let lhs = builder
                    .build_load(i32_type, result, "")
                    .unwrap()
                    .into_int_value();
                let rhs = builder
                    .build_load(i32_type, n, "")
                    .unwrap()
                    .into_int_value();
                let value = builder.build_int_mul(lhs, rhs, "").unwrap();
                builder.build_store(result, value).unwrap();
            }
            {
                let lhs = builder
                    .build_load(i32_type, n, "")
                    .unwrap()
                    .into_int_value();
                let rhs = i32_type.const_int(1, false);
                let value = builder.build_int_sub(lhs, rhs, "").unwrap();
                builder.build_store(n, value).unwrap();
            }
            builder.build_unconditional_branch(loop_block).unwrap();

            // return:
            builder.position_at_end(return_block);
            {
                let value = builder.build_load(i32_type, result, "").unwrap();
                builder.build_return(Some(&value)).unwrap();
            }

            function
        };

        {
            // Main Function Definition
            let name = "main";
            let fn_type = i32_type.fn_type(&[], false);
            let linkage = Some(Linkage::External);
            let function = module.add_function(name, fn_type, linkage);

            let entry_block = context.append_basic_block(function, "entry");

            builder.position_at_end(entry_block);
            let x = {
                let ptr = builder.build_alloca(i32_type, "x").unwrap();
                let value = i32_type.const_int(5, false);
                builder.build_store(ptr, value).unwrap();
                ptr
            };
            let y = {
                let ptr = builder.build_alloca(i32_type, "y").unwrap();
                let value = builder.build_load(i32_type, x, "").unwrap();
                let value = builder
                    .build_call(factorial_fn, &[value.into()], "")
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap();
                builder.build_store(ptr, value).unwrap();
                ptr
            };
            {
                let x = builder.build_load(i32_type, x, "").unwrap();
                let y = builder.build_load(i32_type, y, "").unwrap();
                builder
                    .build_call(
                        printf_fn,
                        &[printf_format_string.into(), x.into(), y.into()],
                        "",
                    )
                    .unwrap();
            }
            {
                let value = i32_type.const_zero();
                builder.build_return(Some(&value)).unwrap();
            }
        };

        module.verify().unwrap();
        module.print_to_stderr();
    }

    #[test]
    #[ignore]
    fn llvm_function_ptr_call_example() {
        let ctx = Context::create();
        let module = ctx.create_module("");
        let builder = ctx.create_builder();

        let ptr_type = ctx.ptr_type(AddressSpace::default());

        let printf_fn = {
            let ty = ctx.i32_type().fn_type(&[ptr_type.into()], true);
            let function = module.add_function("printf", ty, None);
            function
        };

        let (lambda_fn, lambda_ty) = {
            let ty = ctx.i32_type().fn_type(&[], false);
            let function = module.add_function("lambda", ty, None);
            let block = ctx.append_basic_block(function, "");
            builder.position_at_end(block);
            let text = builder
                .build_global_string_ptr("Hello, World!", "")
                .unwrap()
                .as_pointer_value();
            builder.build_call(printf_fn, &[text.into()], "").unwrap();
            let value = ctx.i32_type().const_int(42, false);
            builder.build_return(Some(&value)).unwrap();
            (function, ty)
        };

        {
            let ty = ctx.i32_type().fn_type(&[], false);
            let function = module.add_function("main", ty, None);
            let block = ctx.append_basic_block(function, "");
            builder.position_at_end(block);
            let lambda_ptr = builder.build_alloca(ptr_type, "").unwrap();
            builder
                .build_store(lambda_ptr, lambda_fn.as_global_value())
                .unwrap();
            let lambda_addr = builder
                .build_load(ptr_type, lambda_ptr, "")
                .unwrap()
                .into_pointer_value();
            let value = builder
                .build_indirect_call(lambda_ty, lambda_addr, &[], "")
                .unwrap()
                .try_as_basic_value()
                .left()
                .unwrap();
            builder.build_return(Some(&value)).unwrap();
        }

        module.verify().unwrap();
        module.print_to_stderr();
    }

    #[test]
    #[ignore]
    fn llvm_multi_function_example() {
        let ctx = Context::create();
        let module = ctx.create_module("");

        let fn_ty = ctx.void_type().fn_type(&[], false);

        module.add_function("", fn_ty, None);
        module.add_function("", fn_ty, None);

        module.verify().unwrap();
        module.print_to_stderr();
    }

    #[test]
    #[ignore]
    fn llvm_buffer_overflow_example() {
        let ctx = Context::create();
        let module = ctx.create_module("");
        let builder = ctx.create_builder();

        let ptr_type = ctx.ptr_type(AddressSpace::default());

        let printf_fn = {
            let ty = ctx.i32_type().fn_type(&[ptr_type.into()], true);
            let function = module.add_function("printf", ty, None);
            function
        };

        let i32_ty = ctx.i32_type();
        let i64_ty = ctx.i64_type();
        let fn_ty = ctx.void_type().fn_type(&[], false);

        let func = module.add_function("main", fn_ty, None);
        let block = ctx.append_basic_block(func, "");
        builder.position_at_end(block);
        let a_ptr = {
            let ptr = builder.build_alloca(i32_ty, "a").unwrap();
            builder
                .build_store(ptr, i32_ty.const_int(0xAAAAAAAA, false))
                .unwrap();
            ptr
        };
        let b_ptr = {
            let ptr = builder.build_alloca(i32_ty, "b").unwrap();
            builder
                .build_store(ptr, i32_ty.const_int(0xBBBBBBBB, false))
                .unwrap();
            ptr
        };
        builder
            .build_store(b_ptr, i64_ty.const_int(0xCCCCCCCCCCCCCCCC, false))
            .unwrap();
        let a = builder.build_load(i32_ty, a_ptr, "").unwrap();
        let b = builder.build_load(i32_ty, b_ptr, "").unwrap();
        let format = builder
            .build_global_string_ptr("[%llx] a = 0x%x(%u)\n[%llx] b = 0x%x(%u)\n", "")
            .unwrap()
            .as_pointer_value();
        builder
            .build_call(
                printf_fn,
                &[
                    format.into(),
                    a_ptr.into(),
                    a.into(),
                    a.into(),
                    b_ptr.into(),
                    b.into(),
                    b.into(),
                ],
                "",
            )
            .unwrap();
        builder.build_return(None).unwrap();

        module.verify().unwrap();
        module.print_to_stderr();
    }
}
