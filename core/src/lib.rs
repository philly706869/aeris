pub mod acm;
pub mod syntax;

pub struct AERIS {}

impl AERIS {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod example {
    use inkwell::{AddressSpace, IntPredicate, context::Context, module::Linkage};

    #[test]
    #[ignore]
    fn llvm_example_factorial() {
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
                let ptr = builder.build_alloca(i32_type, "n").unwrap();
                let value = function.get_first_param().unwrap();
                builder.build_store(ptr, value).unwrap();
                ptr
            };
            let result = {
                let ptr = builder.build_alloca(i32_type, "result").unwrap();
                let value = i32_type.const_int(1, false);
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
                let printf_format_string = builder
                    .build_global_string_ptr("fact(%d) = %d\n", "")
                    .unwrap()
                    .as_pointer_value();
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
}
