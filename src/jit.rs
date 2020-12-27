use crate::ast::AstNode;
use libc;
use std::mem;

const PAGE_SIZE: usize = 4096;

type FnPtr = extern "C" fn() -> u64;

struct JitMemory {
    addr: *mut u8,
    raw_addr: *mut libc::c_void,
    size: usize,
    offset: usize,
}

impl JitMemory {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn new(num_pages: usize) -> JitMemory {
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
            addr = mem::transmute(raw_addr);
        }

        JitMemory {
            addr: addr,
            raw_addr: raw_addr,
            size: size,
            offset: 0,
        }
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
        let mut result = 0;

        unsafe {
            self.mark_executable();

            let fn_ptr: FnPtr = mem::transmute(self.addr);

            result = fn_ptr();

            self.mark_writable();
        }

        result
    }

    pub fn fill(&mut self, instructions: &[u8]) {
        for byte in instructions {
            unsafe { *self.addr.offset(self.offset as _) = *byte };
            self.offset += 1;
        }
    }
}

impl Drop for JitMemory {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.addr as *mut _, self.size);
        }
    }
}

pub struct Jit {
    ast: AstNode,
    memory: JitMemory,
}

impl Jit {
    pub fn new(ast: AstNode) -> Self {
        Self {
            ast,
            memory: JitMemory::new(1),
        }
    }

    pub fn execute(&self) {
        self.memory.run();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parse;

    fn get_ast() -> AstNode {
        parse(lex("1 + 2 - 5").unwrap()).unwrap()
    }

    #[test]
    fn jit_new() {
        let ast = get_ast();

        let _ = Jit::new(ast);
    }

    #[test]
    fn jit_execute_code() {
        let mut code: Vec<u8> = vec![
            0x55, //    push   %rbp
            0x48, 0x89, 0xe5, //    mov    %rsp,%rbp
            0xb8, 0x37, 0x00, 0x00, 0x00, //    mov    $0x37,%eax
            0xc9, //    leaveq
            0xc3, //    retq
        ];

        let mut memory = JitMemory::new(1);
        memory.fill(&code);
        assert_eq!(memory.run(), 0x37);
    }
}
