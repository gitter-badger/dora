/* automatically generated by rust-bindgen */

pub type __sig_atomic_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub __val: [::libc::c_ulong; 16usize],
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __sigset_t = Struct_Unnamed1;
pub type sig_atomic_t = __sig_atomic_t;
pub type sigset_t = __sigset_t;
pub type __u_char = ::libc::c_uchar;
pub type __u_short = ::libc::c_ushort;
pub type __u_int = ::libc::c_uint;
pub type __u_long = ::libc::c_ulong;
pub type __int8_t = ::libc::c_char;
pub type __uint8_t = ::libc::c_uchar;
pub type __int16_t = ::libc::c_short;
pub type __uint16_t = ::libc::c_ushort;
pub type __int32_t = ::libc::c_int;
pub type __uint32_t = ::libc::c_uint;
pub type __int64_t = ::libc::c_long;
pub type __uint64_t = ::libc::c_ulong;
pub type __quad_t = ::libc::c_long;
pub type __u_quad_t = ::libc::c_ulong;
pub type __dev_t = ::libc::c_ulong;
pub type __uid_t = ::libc::c_uint;
pub type __gid_t = ::libc::c_uint;
pub type __ino_t = ::libc::c_ulong;
pub type __ino64_t = ::libc::c_ulong;
pub type __mode_t = ::libc::c_uint;
pub type __nlink_t = ::libc::c_ulong;
pub type __off_t = ::libc::c_long;
pub type __off64_t = ::libc::c_long;
pub type __pid_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub __val: [::libc::c_int; 2usize],
}
impl ::std::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __fsid_t = Struct_Unnamed2;
pub type __clock_t = ::libc::c_long;
pub type __rlim_t = ::libc::c_ulong;
pub type __rlim64_t = ::libc::c_ulong;
pub type __id_t = ::libc::c_uint;
pub type __time_t = ::libc::c_long;
pub type __useconds_t = ::libc::c_uint;
pub type __suseconds_t = ::libc::c_long;
pub type __daddr_t = ::libc::c_int;
pub type __key_t = ::libc::c_int;
pub type __clockid_t = ::libc::c_int;
pub type __timer_t = *mut ::libc::c_void;
pub type __blksize_t = ::libc::c_long;
pub type __blkcnt_t = ::libc::c_long;
pub type __blkcnt64_t = ::libc::c_long;
pub type __fsblkcnt_t = ::libc::c_ulong;
pub type __fsblkcnt64_t = ::libc::c_ulong;
pub type __fsfilcnt_t = ::libc::c_ulong;
pub type __fsfilcnt64_t = ::libc::c_ulong;
pub type __fsword_t = ::libc::c_long;
pub type __ssize_t = ::libc::c_long;
pub type __syscall_slong_t = ::libc::c_long;
pub type __syscall_ulong_t = ::libc::c_ulong;
pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut ::libc::c_char;
pub type __intptr_t = ::libc::c_long;
pub type __socklen_t = ::libc::c_uint;
pub type pid_t = __pid_t;
pub type uid_t = __uid_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
impl ::std::clone::Clone for Struct_timespec {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_timespec {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_sigval {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_sigval {
    pub unsafe fn sival_int(&mut self) -> *mut ::libc::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn sival_ptr(&mut self) -> *mut *mut ::libc::c_void {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_sigval {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_sigval {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sigval_t = Union_sigval;
pub type __sigchld_clock_t = __clock_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub si_signo: ::libc::c_int,
    pub si_errno: ::libc::c_int,
    pub si_code: ::libc::c_int,
    pub _sifields: Union_Unnamed4,
}
impl ::std::clone::Clone for Struct_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed4 {
    pub _bindgen_data_: [u64; 14usize],
}
impl Union_Unnamed4 {
    pub unsafe fn _pad(&mut self) -> *mut [::libc::c_int; 28usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _kill(&mut self) -> *mut Struct_Unnamed5 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _timer(&mut self) -> *mut Struct_Unnamed6 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _rt(&mut self) -> *mut Struct_Unnamed7 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _sigchld(&mut self) -> *mut Struct_Unnamed8 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _sigfault(&mut self) -> *mut Struct_Unnamed9 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _sigpoll(&mut self) -> *mut Struct_Unnamed11 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _sigsys(&mut self) -> *mut Struct_Unnamed12 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
impl ::std::clone::Clone for Struct_Unnamed5 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed6 {
    pub si_tid: ::libc::c_int,
    pub si_overrun: ::libc::c_int,
    pub si_sigval: sigval_t,
}
impl ::std::clone::Clone for Struct_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: sigval_t,
}
impl ::std::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::libc::c_int,
    pub si_utime: __sigchld_clock_t,
    pub si_stime: __sigchld_clock_t,
}
impl ::std::clone::Clone for Struct_Unnamed8 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub si_addr: *mut ::libc::c_void,
    pub si_addr_lsb: ::libc::c_short,
    pub si_addr_bnd: Struct_Unnamed10,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed10 {
    pub _lower: *mut ::libc::c_void,
    pub _upper: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct_Unnamed10 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed11 {
    pub si_band: ::libc::c_long,
    pub si_fd: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed11 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed11 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub _call_addr: *mut ::libc::c_void,
    pub _syscall: ::libc::c_int,
    pub _arch: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type siginfo_t = Struct_Unnamed3;
#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum_Unnamed13 {
    SI_ASYNCNL = -60,
    SI_TKILL = -6,
    SI_SIGIO = -5,
    SI_ASYNCIO = -4,
    SI_MESGQ = -3,
    SI_TIMER = -2,
    SI_QUEUE = -1,
    SI_USER = 0,
    SI_KERNEL = 128,
}
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed14 {
    ILL_ILLOPC = 1,
    ILL_ILLOPN = 2,
    ILL_ILLADR = 3,
    ILL_ILLTRP = 4,
    ILL_PRVOPC = 5,
    ILL_PRVREG = 6,
    ILL_COPROC = 7,
    ILL_BADSTK = 8,
}
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed15 {
    FPE_INTDIV = 1,
    FPE_INTOVF = 2,
    FPE_FLTDIV = 3,
    FPE_FLTOVF = 4,
    FPE_FLTUND = 5,
    FPE_FLTRES = 6,
    FPE_FLTINV = 7,
    FPE_FLTSUB = 8,
}
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed16 { SEGV_MAPERR = 1, SEGV_ACCERR = 2, }
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed17 {
    BUS_ADRALN = 1,
    BUS_ADRERR = 2,
    BUS_OBJERR = 3,
    BUS_MCEERR_AR = 4,
    BUS_MCEERR_AO = 5,
}
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed18 {
    CLD_EXITED = 1,
    CLD_KILLED = 2,
    CLD_DUMPED = 3,
    CLD_TRAPPED = 4,
    CLD_STOPPED = 5,
    CLD_CONTINUED = 6,
}
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed19 {
    POLL_IN = 1,
    POLL_OUT = 2,
    POLL_MSG = 3,
    POLL_ERR = 4,
    POLL_PRI = 5,
    POLL_HUP = 6,
}
pub type pthread_attr_t = Union_pthread_attr_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sigevent {
    pub sigev_value: sigval_t,
    pub sigev_signo: ::libc::c_int,
    pub sigev_notify: ::libc::c_int,
    pub _sigev_un: Union_Unnamed20,
}
impl ::std::clone::Clone for Struct_sigevent {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sigevent {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed20 {
    pub _bindgen_data_: [u64; 6usize],
}
impl Union_Unnamed20 {
    pub unsafe fn _pad(&mut self) -> *mut [::libc::c_int; 12usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _tid(&mut self) -> *mut __pid_t {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _sigev_thread(&mut self) -> *mut Struct_Unnamed21 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed20 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed20 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed21 {
    pub _function: ::std::option::Option<extern "C" fn(arg1: sigval_t)>,
    pub _attribute: *mut pthread_attr_t,
}
impl ::std::clone::Clone for Struct_Unnamed21 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed21 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sigevent_t = Struct_sigevent;
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed22 {
    SIGEV_SIGNAL = 0,
    SIGEV_NONE = 1,
    SIGEV_THREAD = 2,
    SIGEV_THREAD_ID = 4,
}
pub type __sighandler_t =
    ::std::option::Option<extern "C" fn(arg1: ::libc::c_int)>;
pub type sig_t = __sighandler_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sigaction {
    pub __sigaction_handler: Union_Unnamed23,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::libc::c_int,
    pub sa_restorer: ::std::option::Option<extern "C" fn()>,
}
impl ::std::clone::Clone for Struct_sigaction {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sigaction {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed23 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed23 {
    pub unsafe fn sa_handler(&mut self) -> *mut __sighandler_t {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn sa_sigaction(&mut self)
     ->
         *mut ::std::option::Option<unsafe extern "C" fn(arg1: ::libc::c_int,
                                                         arg2: *mut siginfo_t,
                                                         arg3:
                                                             *mut ::libc::c_void)> {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed23 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed23 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__fpx_sw_bytes {
    pub magic1: __uint32_t,
    pub extended_size: __uint32_t,
    pub xstate_bv: __uint64_t,
    pub xstate_size: __uint32_t,
    pub padding: [__uint32_t; 7usize],
}
impl ::std::clone::Clone for Struct__fpx_sw_bytes {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__fpx_sw_bytes {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__fpreg {
    pub significand: [::libc::c_ushort; 4usize],
    pub exponent: ::libc::c_ushort,
}
impl ::std::clone::Clone for Struct__fpreg {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__fpreg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__fpxreg {
    pub significand: [::libc::c_ushort; 4usize],
    pub exponent: ::libc::c_ushort,
    pub padding: [::libc::c_ushort; 3usize],
}
impl ::std::clone::Clone for Struct__fpxreg {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__fpxreg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__xmmreg {
    pub element: [__uint32_t; 4usize],
}
impl ::std::clone::Clone for Struct__xmmreg {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__xmmreg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [Struct__fpxreg; 8usize],
    pub _xmm: [Struct__xmmreg; 16usize],
    pub padding: [__uint32_t; 24usize],
}
impl ::std::clone::Clone for Struct__fpstate {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__fpstate {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sigcontext {
    pub r8: __uint64_t,
    pub r9: __uint64_t,
    pub r10: __uint64_t,
    pub r11: __uint64_t,
    pub r12: __uint64_t,
    pub r13: __uint64_t,
    pub r14: __uint64_t,
    pub r15: __uint64_t,
    pub rdi: __uint64_t,
    pub rsi: __uint64_t,
    pub rbp: __uint64_t,
    pub rbx: __uint64_t,
    pub rdx: __uint64_t,
    pub rax: __uint64_t,
    pub rcx: __uint64_t,
    pub rsp: __uint64_t,
    pub rip: __uint64_t,
    pub eflags: __uint64_t,
    pub cs: ::libc::c_ushort,
    pub gs: ::libc::c_ushort,
    pub fs: ::libc::c_ushort,
    pub __pad0: ::libc::c_ushort,
    pub err: __uint64_t,
    pub trapno: __uint64_t,
    pub oldmask: __uint64_t,
    pub cr2: __uint64_t,
    pub _bindgen_data_1_: [u64; 1usize],
    pub __reserved1: [__uint64_t; 8usize],
}
impl Struct_sigcontext {
    pub unsafe fn fpstate(&mut self) -> *mut *mut Struct__fpstate {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __fpstate_word(&mut self) -> *mut __uint64_t {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Struct_sigcontext {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sigcontext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__xsave_hdr {
    pub xstate_bv: __uint64_t,
    pub reserved1: [__uint64_t; 2usize],
    pub reserved2: [__uint64_t; 5usize],
}
impl ::std::clone::Clone for Struct__xsave_hdr {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__xsave_hdr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__ymmh_state {
    pub ymmh_space: [__uint32_t; 64usize],
}
impl ::std::clone::Clone for Struct__ymmh_state {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__ymmh_state {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__xstate {
    pub fpstate: Struct__fpstate,
    pub xstate_hdr: Struct__xsave_hdr,
    pub ymmh: Struct__ymmh_state,
}
impl ::std::clone::Clone for Struct__xstate {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__xstate {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type size_t = ::libc::c_ulong;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sigstack {
    pub ss_sp: *mut ::libc::c_void,
    pub ss_onstack: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_sigstack {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sigstack {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed24 { SS_ONSTACK = 1, SS_DISABLE = 2, }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sigaltstack {
    pub ss_sp: *mut ::libc::c_void,
    pub ss_flags: ::libc::c_int,
    pub ss_size: size_t,
}
impl ::std::clone::Clone for Struct_sigaltstack {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sigaltstack {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type stack_t = Struct_sigaltstack;
pub type pthread_t = ::libc::c_ulong;
#[repr(C)]
#[derive(Copy)]
pub struct Union_pthread_attr_t {
    pub _bindgen_data_: [u64; 7usize],
}
impl Union_pthread_attr_t {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 56usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_pthread_attr_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_pthread_attr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___pthread_internal_list {
    pub __prev: *mut Struct___pthread_internal_list,
    pub __next: *mut Struct___pthread_internal_list,
}
impl ::std::clone::Clone for Struct___pthread_internal_list {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct___pthread_internal_list {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __pthread_list_t = Struct___pthread_internal_list;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed25 {
    pub _bindgen_data_: [u64; 5usize],
}
impl Union_Unnamed25 {
    pub unsafe fn __data(&mut self) -> *mut Struct___pthread_mutex_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 40usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed25 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed25 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct___pthread_mutex_s {
    pub __lock: ::libc::c_int,
    pub __count: ::libc::c_uint,
    pub __owner: ::libc::c_int,
    pub __nusers: ::libc::c_uint,
    pub __kind: ::libc::c_int,
    pub __spins: ::libc::c_short,
    pub __elision: ::libc::c_short,
    pub __list: __pthread_list_t,
}
impl ::std::clone::Clone for Struct___pthread_mutex_s {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct___pthread_mutex_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_mutex_t = Union_Unnamed25;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed26 {
    pub _bindgen_data_: [u32; 1usize],
}
impl Union_Unnamed26 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed26 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed26 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_mutexattr_t = Union_Unnamed26;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed27 {
    pub _bindgen_data_: [u64; 6usize],
}
impl Union_Unnamed27 {
    pub unsafe fn __data(&mut self) -> *mut Struct_Unnamed28 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 48usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_longlong {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed27 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed27 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed28 {
    pub __lock: ::libc::c_int,
    pub __futex: ::libc::c_uint,
    pub __total_seq: ::libc::c_ulonglong,
    pub __wakeup_seq: ::libc::c_ulonglong,
    pub __woken_seq: ::libc::c_ulonglong,
    pub __mutex: *mut ::libc::c_void,
    pub __nwaiters: ::libc::c_uint,
    pub __broadcast_seq: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_Unnamed28 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed28 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_cond_t = Union_Unnamed27;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed29 {
    pub _bindgen_data_: [u32; 1usize],
}
impl Union_Unnamed29 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed29 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed29 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_condattr_t = Union_Unnamed29;
pub type pthread_key_t = ::libc::c_uint;
pub type pthread_once_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed30 {
    pub _bindgen_data_: [u64; 7usize],
}
impl Union_Unnamed30 {
    pub unsafe fn __data(&mut self) -> *mut Struct_Unnamed31 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 56usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed30 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed30 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed31 {
    pub __lock: ::libc::c_int,
    pub __nr_readers: ::libc::c_uint,
    pub __readers_wakeup: ::libc::c_uint,
    pub __writer_wakeup: ::libc::c_uint,
    pub __nr_readers_queued: ::libc::c_uint,
    pub __nr_writers_queued: ::libc::c_uint,
    pub __writer: ::libc::c_int,
    pub __shared: ::libc::c_int,
    pub __rwelision: ::libc::c_char,
    pub __pad1: [::libc::c_uchar; 7usize],
    pub __pad2: ::libc::c_ulong,
    pub __flags: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_Unnamed31 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed31 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_rwlock_t = Union_Unnamed30;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed32 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed32 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 8usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed32 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed32 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_rwlockattr_t = Union_Unnamed32;
pub type pthread_spinlock_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed33 {
    pub _bindgen_data_: [u64; 4usize],
}
impl Union_Unnamed33 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 32usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed33 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed33 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_barrier_t = Union_Unnamed33;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed34 {
    pub _bindgen_data_: [u32; 1usize],
}
impl Union_Unnamed34 {
    pub unsafe fn __size(&mut self) -> *mut [::libc::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::libc::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed34 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed34 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_barrierattr_t = Union_Unnamed34;
pub type greg_t = ::libc::c_longlong;
pub type gregset_t = [greg_t; 23usize];
#[repr(C)]
#[derive(Copy)]
pub struct Struct__libc_fpxreg {
    pub significand: [::libc::c_ushort; 4usize],
    pub exponent: ::libc::c_ushort,
    pub padding: [::libc::c_ushort; 3usize],
}
impl ::std::clone::Clone for Struct__libc_fpxreg {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__libc_fpxreg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__libc_xmmreg {
    pub element: [__uint32_t; 4usize],
}
impl ::std::clone::Clone for Struct__libc_xmmreg {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__libc_xmmreg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct__libc_fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [Struct__libc_fpxreg; 8usize],
    pub _xmm: [Struct__libc_xmmreg; 16usize],
    pub padding: [__uint32_t; 24usize],
}
impl ::std::clone::Clone for Struct__libc_fpstate {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__libc_fpstate {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type fpregset_t = *mut Struct__libc_fpstate;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed35 {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [::libc::c_ulonglong; 8usize],
}
impl ::std::clone::Clone for Struct_Unnamed35 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed35 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type mcontext_t = Struct_Unnamed35;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ucontext {
    pub uc_flags: ::libc::c_ulong,
    pub uc_link: *mut Struct_ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: __sigset_t,
    pub __fpregs_mem: Struct__libc_fpstate,
}
impl ::std::clone::Clone for Struct_ucontext {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ucontext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ucontext_t = Struct_ucontext;
extern "C" {
    pub static mut _sys_siglist: [*const ::libc::c_char; 65usize];
    pub static mut sys_siglist: [*const ::libc::c_char; 65usize];
}
extern "C" {
    pub fn __sigismember(arg1: *const __sigset_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn __sigaddset(arg1: *mut __sigset_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn __sigdelset(arg1: *mut __sigset_t, arg2: ::libc::c_int)
     -> ::libc::c_int;
    pub fn __sysv_signal(__sig: ::libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    pub fn signal(__sig: ::libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    pub fn kill(__pid: __pid_t, __sig: ::libc::c_int) -> ::libc::c_int;
    pub fn killpg(__pgrp: __pid_t, __sig: ::libc::c_int) -> ::libc::c_int;
    pub fn raise(__sig: ::libc::c_int) -> ::libc::c_int;
    pub fn ssignal(__sig: ::libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    pub fn gsignal(__sig: ::libc::c_int) -> ::libc::c_int;
    pub fn psignal(__sig: ::libc::c_int, __s: *const ::libc::c_char);
    pub fn psiginfo(__pinfo: *const siginfo_t, __s: *const ::libc::c_char);
    pub fn sigblock(__mask: ::libc::c_int) -> ::libc::c_int;
    pub fn sigsetmask(__mask: ::libc::c_int) -> ::libc::c_int;
    pub fn siggetmask() -> ::libc::c_int;
    pub fn sigemptyset(__set: *mut sigset_t) -> ::libc::c_int;
    pub fn sigfillset(__set: *mut sigset_t) -> ::libc::c_int;
    pub fn sigaddset(__set: *mut sigset_t, __signo: ::libc::c_int)
     -> ::libc::c_int;
    pub fn sigdelset(__set: *mut sigset_t, __signo: ::libc::c_int)
     -> ::libc::c_int;
    pub fn sigismember(__set: *const sigset_t, __signo: ::libc::c_int)
     -> ::libc::c_int;
    pub fn sigprocmask(__how: ::libc::c_int, __set: *const sigset_t,
                       __oset: *mut sigset_t) -> ::libc::c_int;
    pub fn sigsuspend(__set: *const sigset_t) -> ::libc::c_int;
    pub fn sigaction(__sig: ::libc::c_int, __act: *const Struct_sigaction,
                     __oact: *mut Struct_sigaction) -> ::libc::c_int;
    pub fn sigpending(__set: *mut sigset_t) -> ::libc::c_int;
    pub fn sigwait(__set: *const sigset_t, __sig: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn sigwaitinfo(__set: *const sigset_t, __info: *mut siginfo_t)
     -> ::libc::c_int;
    pub fn sigtimedwait(__set: *const sigset_t, __info: *mut siginfo_t,
                        __timeout: *const Struct_timespec) -> ::libc::c_int;
    pub fn sigqueue(__pid: __pid_t, __sig: ::libc::c_int, __val: Union_sigval)
     -> ::libc::c_int;
    pub fn sigreturn(__scp: *mut Struct_sigcontext) -> ::libc::c_int;
    pub fn siginterrupt(__sig: ::libc::c_int, __interrupt: ::libc::c_int)
     -> ::libc::c_int;
    pub fn sigstack(__ss: *mut Struct_sigstack, __oss: *mut Struct_sigstack)
     -> ::libc::c_int;
    pub fn sigaltstack(__ss: *const Struct_sigaltstack,
                       __oss: *mut Struct_sigaltstack) -> ::libc::c_int;
    pub fn pthread_sigmask(__how: ::libc::c_int, __newmask: *const __sigset_t,
                           __oldmask: *mut __sigset_t) -> ::libc::c_int;
    pub fn pthread_kill(__threadid: pthread_t, __signo: ::libc::c_int)
     -> ::libc::c_int;
    pub fn __libc_current_sigrtmin() -> ::libc::c_int;
    pub fn __libc_current_sigrtmax() -> ::libc::c_int;
}
