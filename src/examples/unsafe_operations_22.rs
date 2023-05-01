pub fn unsafe_operations() {
    let some_vector = vec![1, 2, 3, 4];
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();
    unsafe {
        let my_slice: &[u32] = std::slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), my_slice);
    }
}

pub fn inline_assembly() {
    use std::arch::asm;

    // ===================== basic usage ======================
    // unsafe {
    //     asm!("nop");
    // }

    // ===================== inputs and outputs ======================
    // let x: u64;
    // unsafe {
    //     asm!("mov {}, 5", out(reg) x);
    // }
    // assert_eq!(x, 5);
    // --------------------------------------------------
    // let i: u64 = 3;
    // let o: u64;
    // unsafe {
    //     asm!(
    //         "mov {0}, {1}",
    //         "add {0}, 5",
    //         out(reg) o,
    //         in(reg) i,
    //     )
    // }
    // assert_eq!(o, 8);
    // ---------------------------------------------------
    // let mut x: u64 = 3;
    // unsafe {
    //     asm!("add {0}, 5", inout(reg) x);
    // }
    // assert_eq!(x, 8);

    // ===================== late output operands ======================
    // let cmd = 0xd1;
    // unsafe {
    //     asm!("out 0x64, eax", in("eax") cmd);
    // }
    // ------------------------------------------------
    // fn mul(a: u64, b: u64) -> u128 {
    //     let lo: u64;
    //     let hi: u64;
    //     unsafe {
    //         asm!(
    //             "mul {}",
    //             in(reg) a,
    //             inlateout("rax") b => lo,
    //             lateout("rdx") hi,
    //         )
    //     }
    //     ((hi as u128) << 64) + lo as u128
    // }
    // ------------------------------------------------
    // let mut name_buf = [0_u8; 12];
    // unsafe {
    //     asm!(
    //         "push rbx",
    //         "cpuid",
    //         "mov [rdi], ebx",
    //         "mov [rdi + 4], edx",
    //         "mov [rdi + 8], ecx",
    //         "pop rbx",
    //         in("rdi") name_buf.as_mut_ptr(),
    //         inout("eax") 0 => _,
    //         out("ecx") _,
    //         out("edx") _,
    //     );
    // }
    // let name = core::str::from_utf8(&name_buf).unwrap();
    // println!("cpu manufacturer id: {}", name);
    // ------------------------------------------------
    // let mut x: u64 = 4;
    // unsafe {
    //     asm!(
    //         "mov {tmp}, {x}",
    //         "shl {tmp}, 1",
    //         "shl {x}, 2",
    //         "add {x}, {tmp}",
    //         x = inout(reg) x,
    //         tmp = out(reg) _,
    //     );
    // }
    // assert_eq!(x, 4 * 6);

    // ===================== symbol operands and ABI clobbers ======================
    // extern "C" fn foo(arg: i32) -> i32 {
    //     println!("arg = {}", arg);
    //     arg * 2
    // }
    // fn call_foo(arg: i32) -> i32 {
    //     unsafe {
    //         let result;
    //         asm!(
    //             "call {}",
    //             in(reg) foo,
    //             in("rdi") arg,
    //             out("rax") result,
    //             clobber_abi("C"),
    //         );
    //         result
    //     }
    // }

    // ===================== register template modifiers ======================
    // let mut x: u16 = 0xab;
    // unsafe {
    //     asm!("mov {0:h}, {0:l}", inout(reg_abcd) x);
    // }
    // assert_eq!(x, 0xabab);

    // ===================== memory address operands ======================
    // fn load_fpu_control_word(control: u16) {
    //     unsafe {
    //         asm!("fldcw [{}]", in(reg) &control, options(nostack));
    //     }
    // }

    // ===================== labels ======================
    // let mut a = 0;
    // unsafe {
    //     asm!(
    //         "mov {0}, 10",
    //         "2:",
    //         "sub {0}, 1",
    //         "cmp {0}, 3",
    //         "jle 2f",
    //         "jmp 2b",
    //         "2:",
    //         "add {0}, 2",
    //         out(reg) a
    //     );
    // }
    // assert_eq!(a, 5);

    // ===================== options ======================
    let mut a: u64 = 4;
    let b: u64 = 4;
    unsafe {
        asm!(
            "add {0}, {1}",
            inlateout(reg) a, in(reg) b,
            options(pure, nomem, nostack),
        );
    }
    assert_eq!(a, 8);
}

pub fn unsafe_operations_22() {
    // unsafe_operations();
    inline_assembly();
}
