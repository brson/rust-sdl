use libc::c_int;


/** Enumeration of valid key mods (possibly OR'd together) */
pub enum Mod {
    pub KMODNone            = 0x0000,
    pub KMODLShift          = 0x0001,
    pub KMODRShift          = 0x0002,
    pub KMODLCtrl           = 0x0040,
    pub KMODRCtrl           = 0x0080,
    pub KMODLAlt            = 0x0100,
    pub KMODRAlt            = 0x0200,
    pub KMODLMeta           = 0x0400,
    pub KMODRMeta           = 0x0800,
    pub KMODNum             = 0x1000,
    pub KMODCaps            = 0x2000,
    pub KMODMode            = 0x4000,
    pub KMODReserved        = 0x8000
}
