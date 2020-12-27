use std::mem;

const PAGE_SIZE: usize = 4096;

type FnPtr = extern "C" fn() -> u64;

pub struct Jit {
    addr: *mut u8,
    raw_addr: *mut libc::c_void,
    size: usize,
    offset: usize,
}

impl Jit {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn new(instructions: &[u8]) -> Jit {
        let num_pages = (instructions.len() as f32 / PAGE_SIZE as f32).ceil() as usize;
        let size: usize = num_pages * PAGE_SIZE;
        let addr: *mut u8;
        let mut raw_addr: *mut libc::c_void;

        unsafe {
            // Take a pointer
            raw_addr = mem::MaybeUninit::zeroed().assume_init();

            // Allocate aligned to page size
            libc::posix_memalign(&mut raw_addr, PAGE_SIZE, size);

            // Mark the memory as read-write
            libc::mprotect(raw_addr, size, libc::PROT_READ | libc::PROT_WRITE);

            // Fill with 'RET' calls (0xc3)
            libc::memset(raw_addr, 0xc3, size);

            // Transmute the c_void pointer to a Rust u8 pointer
            addr = raw_addr as *mut u8;
        }

        let mut jit = Jit {
            addr,
            raw_addr,
            size,
            offset: 0,
        };

        jit.write_instructions(instructions);

        jit
    }

    fn mark_writable(&self) {
        unsafe {
            libc::mprotect(self.raw_addr, self.size, libc::PROT_READ | libc::PROT_WRITE);
        }
    }

    fn mark_executable(&self) {
        unsafe {
            libc::mprotect(self.raw_addr, self.size, libc::PROT_EXEC);
        }
    }

    pub fn run(&self) -> u64 {
        let result;

        unsafe {
            self.mark_executable();

            let fn_ptr: FnPtr = mem::transmute(self.addr);

            result = fn_ptr();

            self.mark_writable();
        }

        result
    }

    fn write_instructions(&mut self, instructions: &[u8]) {
        for byte in instructions {
            unsafe { *self.addr.add(self.offset) = *byte };
            self.offset += 1;
        }
    }
}

impl Drop for Jit {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.addr as *mut _, self.size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::gen_assembly;
    use crate::lexer::lex;
    use crate::parser::parse;

    fn get_asm(input: &str) -> Vec<u8> {
        let asm = gen_assembly(parse(lex(input).unwrap()).unwrap()).unwrap();
        println!("{:02x?}", asm);
        asm
    }

    #[test]
    fn jit_new() {
        let _ = Jit::new(&get_asm("12"));
    }

    #[test]
    fn jit_execute_code() {
        let code: Vec<u8> = vec![
            0x55, //    push   %rbp
            0x48, 0x89, 0xe5, //    mov    %rsp,%rbp
            0xb8, 0x37, 0x00, 0x00, 0x00, //    mov    $0x37,%eax
            0xc9, //    leaveq
            0xc3, //    retq
        ];

        let memory = Jit::new(&code);
        assert_eq!(memory.run(), 0x37);
    }

    #[test]
    fn jit_execute_single_int() {
        let jit = Jit::new(&get_asm("42"));
        assert_eq!(jit.run(), 42);
    }

    #[test]
    fn jit_execute_add_expression() {
        let jit = Jit::new(&get_asm("42 + 12"));
        assert_eq!(jit.run(), 54);
    }

    #[test]
    fn jit_execute_subtract_expression() {
        let jit = Jit::new(&get_asm("42 - 12"));
        assert_eq!(jit.run(), 30);
    }

    #[test]
    fn jit_execute_add_subtract_expression() {
        let jit = Jit::new(&get_asm("42 - 12 + 12 - 5 + 2284"));
        assert_eq!(jit.run(), 2321);
    }

    #[test]
    fn jit_execute_multiply_expression() {
        let jit = Jit::new(&get_asm("2 * 4 * 3"));
        assert_eq!(jit.run(), 24);
    }

    #[test]
    fn jit_execute_divide_expression() {
        let jit = Jit::new(&get_asm("16 / 4 / 2"));
        assert_eq!(jit.run(), 2);
    }
}
