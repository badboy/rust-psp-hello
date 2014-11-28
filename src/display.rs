use libc::c_int;

#[link(name = "pspdisplay")]
#[allow(improper_ctypes)]
extern {
    fn sceDisplayWaitVblankStart() -> c_int;
    fn sceDisplayWaitVblank() -> c_int;
}

// Wait for vertical blank start
pub fn wait_vblank_start() -> i32 {
    unsafe { sceDisplayWaitVblankStart() as i32 }
}
