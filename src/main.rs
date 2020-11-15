use std::mem;
use exp_jit::Asm;
use byteorder::{LittleEndian, ByteOrder};

#[no_mangle]
pub extern "C" fn foo() -> u64 {
    1024
}

fn main() {
    let foo_pointer = foo as *const () as u64;
    let mut asm = Asm::new();
    //movabs rax, foo_ptr
    asm.put(&[0x48, 0xB8]);
    asm.put(&encode_ptr(foo_pointer));
    // call rax
    asm.put(&[0xFF, 0xD0]);
    //ret
    asm.put(&[0xC3]);
    let func = unsafe { asm.prepare::<fn() -> u64>().unwrap() };
    let asm_foo = unsafe { func.func() };
    assert_eq!(foo(), asm_foo());
}

fn encode_ptr(ptr: u64) -> [u8; 8] {
    let mut f_prt = [0; 8];
    LittleEndian::write_u64(&mut f_prt, ptr);
    f_prt
}
