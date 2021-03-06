extern crate libc;

use libc::*;
use ucontext::{ucontext_t, mcontext_t};

mod ucontext;

static REG_RBP: usize = 10;
static REG_RSP: usize = 15;
static REG_RIP: usize = 16;

type Fct = extern "C" fn() -> i32;

fn main() {
    unsafe {
        let mut sa: sigaction = std::mem::uninitialized();

        sa.sa_sigaction = handler as usize;
        sigemptyset(&mut sa.sa_mask as *mut sigset_t);
        sa.sa_flags = SA_SIGINFO;

        if sigaction(SIGSEGV, &sa as *const sigaction, 0 as *mut sigaction) == -1 {
            perror("sigaction failed".as_ptr() as *const i8);
        }

        // int fct2() { return 4; }
        // int fct2_stub() { <FAIL> }
        // int fct1() { return fct2(); }

        // compiler_stub: mov r10, [9]
        let fct2_stub_code = [ 0x4C, 0x8B, 0x14, 0x25, 9, 0, 0, 0 ];
        let fct2_stub = alloc_code(&fct2_stub_code);
        dump("fct2_stub", fct2_stub as usize, fct2_stub_code.len());

        // (int3)
        // push rbp
        // movabs rax, 0x1122334455667788
        // call rax
        // pop rbp
        // ret
        let fct1_code = [
            0x55,
            0x48, 0xB8, 0, 0, 0, 0, 0, 0, 0, 0,
            0xFF, 0xD0,
            0x5D,
            0xC3
        ];

        let fct1 = alloc_code(&fct1_code);
        dump("fct1", fct1 as usize, fct1_code.len());

        let patcher = (fct1 as usize) + 3;
        let patcher = patcher as *mut usize;
        *patcher = fct2_stub as usize;

        dump("fct1 patched (1)", fct1 as usize, fct1_code.len());

        println!("invoke fct1:");
        let res1 = fct1();
        println!("res = {}", res1);

        // invoke fct a second time - stub should not be used anymore
        println!("\ninvoke fct1 again:");
        let res2 = fct1();
        println!("res = {}", res2);
    }
}

fn handler(signo: c_int, _: *const siginfo_t, ucontext: *mut ucontext_t) {
    println!("SIGNAL {}!", signo);

    unsafe {
        let mcontext = &mut (*ucontext).uc_mcontext;

        let rip = mcontext.gregs[REG_RIP] as usize;
        dump("\trip", rip, 8);

        let rbp = mcontext.gregs[REG_RBP] as usize;
        println!("\trbp = {:x}", rbp);

        let rsp = mcontext.gregs[REG_RSP] as usize;
        println!("\trsp = {:x}", rsp);

        let ra = *(rsp as *mut usize);
        println!("\treturn address = {:x}\n", ra);

        let ra = ra - 12;
        println!("\treturn address before call {:x}\n", ra);

        // push rbp
        // mov eax, 4
        // pop rbp
        // ret
        let code = [
            0x55,
            0xb8, 4, 0, 0, 0,
            0x5D,
            0xc3
        ];

        let fct2 = alloc_code(&code);
        dump("\tfct2", fct2 as usize, code.len());

        let patcher = (ra + 2) as *mut usize;
        *patcher = fct2 as usize;

        dump("\tfct1 patched (2)", ra-1, 15);

        mcontext.gregs[REG_RIP] = ra as i64;
        mcontext.gregs[REG_RSP] = (rsp + 8) as i64;
    }
}

fn alloc_code(code: &[u8]) -> Fct {
    unsafe {
        let pagesize = sysconf(_SC_PAGESIZE) as usize;

        let ptr = mmap(0 as *mut c_void, pagesize,
                       PROT_READ | PROT_WRITE | PROT_EXEC,
                       MAP_ANON | MAP_PRIVATE, -1, 0);

        std::ptr::copy_nonoverlapping(code.as_ptr(), ptr as *mut u8, code.len());

        std::mem::transmute(ptr)
    }
}

fn dump(name: &'static str, ptr: usize, len: usize) {
    print!("{} @ {:x} ({} bytes) = ", name, ptr, len);

    unsafe {
        let ptr = ptr as *const u8;

        for i in 0..len {
            print!("{:02x} ", *ptr.offset(i as isize));
        }
    }

    println!("");
}

