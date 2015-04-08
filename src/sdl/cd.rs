use libc::c_int;
use std::str;
use std::ffi::CStr;

use get_error;

pub mod ll {
    #![allow(non_camel_case_types)]

    use libc::{c_int, uint8_t, uint16_t, uint32_t};
    use libc::types::os::arch::c95::c_schar;

    pub type CDstatus = c_int;

    pub const CD_TRAYEMPTY: CDstatus = 0;
    pub const CD_STOPPED: CDstatus = 1;
    pub const CD_PLAYING: CDstatus = 2;
    pub const CD_PAUSED: CDstatus = 3;
    pub const CD_ERROR: CDstatus = -1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_CDtrack {
        pub id: uint8_t,
        pub _type: uint8_t,
        pub unused: uint16_t,
        pub length: uint32_t,
        pub offset: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    pub struct SDL_CD {
        pub id: c_int,
        pub status: CDstatus,
        pub numtracks: c_int,
        pub cur_track: c_int,
        pub cur_frame: c_int,
        pub track: [SDL_CDtrack; 100],
    }

    impl Clone for SDL_CD {
        fn clone(&self) -> SDL_CD {
            *self
        }
    }

    extern "C" {
        pub fn SDL_CDNumDrives() -> c_int;
        pub fn SDL_CDName(drive: c_int) -> *const c_schar;
        pub fn SDL_CDOpen(drive: c_int) -> *mut SDL_CD;
        pub fn SDL_CDStatus(cdrom: *mut SDL_CD) -> CDstatus;
        pub fn SDL_CDClose(cdrom: *mut SDL_CD);
        pub fn SDL_CDStop(cdrom: *mut SDL_CD) -> c_int;
        pub fn SDL_CDEject(cdrom: *mut SDL_CD) -> c_int;
        pub fn SDL_CDResume(cdrom: *mut SDL_CD) -> c_int;
        pub fn SDL_CDPlay(cdrom: *mut SDL_CD, start: c_int, length: c_int) -> c_int;
        pub fn SDL_CDPlayTracks(cdrom: *mut SDL_CD,
                                start_track: c_int,
                                start_frame: c_int,
                                ntracks: c_int,
                                nframes: c_int) -> c_int;
        pub fn SDL_CDPause(cdrom: *mut SDL_CD) -> c_int;
    }
}

pub fn get_num_drives() -> isize {
    unsafe { ll::SDL_CDNumDrives() as isize }
}

pub fn get_drive_name(index: isize) -> Result<String, String> {
    unsafe {
        let cstr = ll::SDL_CDName(index as c_int);

        if cstr.is_null() {
            Err(get_error())
        } else {
            Ok(str::from_utf8(CStr::from_ptr(cstr).to_bytes()).unwrap().to_string())
        }
    }
}

#[derive(PartialEq)]
pub struct CD {
    pub raw: *mut ll::SDL_CD
}

fn wrap_cd(raw: *mut ll::SDL_CD) -> CD {
    CD { raw: raw }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Status {
    TrayEmpty = ll::CD_TRAYEMPTY as isize,
    Stopped = ll::CD_STOPPED as isize,
    Playing = ll::CD_PLAYING as isize,
    Paused = ll::CD_PAUSED as isize,
    Error = ll::CD_ERROR as isize
}

impl CD {
    pub fn open(index: isize) -> Result<CD, String> {
        unsafe {
            let raw = ll::SDL_CDOpen(index as c_int);

            if raw.is_null() { Err(get_error()) }
            else { Ok(wrap_cd(raw)) }
        }
    }

    pub fn get_status(&self) -> Status {
        unsafe {
            // FIXME: Rust doesn't like us matching using staticants here for some reason
            match ll::SDL_CDStatus(self.raw) {
                0 => Status::TrayEmpty,
                1 => Status::Stopped,
                2 => Status::Playing,
                3 => Status::Paused,
                -1 => Status::Error,
                _ => Status::Error
            }
        }
    }

    pub fn play(&self, start: isize, len: isize) -> bool {
        unsafe { ll::SDL_CDPlay(self.raw, start as c_int, len as c_int) == 0 }
    }

    pub fn play_tracks(&self, start_track: isize, start_frame: isize, ntracks: isize,
                   nframes: isize) -> bool {
        unsafe {
            ll::SDL_CDPlayTracks(self.raw, start_track as c_int, start_frame as c_int,
                                 ntracks as c_int, nframes as c_int) == 0
        }
    }

    pub fn pause(&self) -> bool {
        unsafe { ll::SDL_CDPause(self.raw) == 0 }
    }

    pub fn resume(&self) -> bool {
        unsafe { ll::SDL_CDResume(self.raw) == 0 }
    }

    pub fn stop(&self) -> bool {
        unsafe { ll::SDL_CDStop(self.raw) == 0 }
    }
}

impl Drop for CD {
    fn drop(&mut self) {
        unsafe { ll::SDL_CDClose(self.raw); }
    }
}
