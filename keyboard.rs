use libc::c_int;

// sym corresponds to the `Key` enum, mod_ to the `Mod` enum. We should
// be using the correct type here but our enums don't have the right
// size yet
pub type KeySym = {
    scancode: u8,
    sym: c_int,
    mod_: c_int,
    unicode: u16
};

/** What we really want is a mapping of every raw key on the keyboard.
 *  To support international keyboards, we use the range 0xA1 - 0xFF
 *  as international virtual keycodes.  We'll follow in the footsteps of X11...
 *  @brief The names of the keys
 */
pub enum Key {
    /** @name ASCII mapped KeySyms
    *  The keyboard syms have been cleverly chosen to map to ASCII
    */
    /*@{*/
    pub SDLKUnknown         = 0,
    //pub SDLKFirst         = 0,
    pub SDLKBackspace       = 8,
    pub SDLKTab             = 9,
    pub SDLKClear           = 12,
    pub SDLKReturn          = 13,
    pub SDLKPause           = 19,
    pub SDLKEscape          = 27,
    pub SDLKSpace           = 32,
    pub SDLKExclaim         = 33,
    pub SDLKQuotedbl        = 34,
    pub SDLKHash            = 35,
    pub SDLKDollar          = 36,
    pub SDLKAmpersand       = 38,
    pub SDLKQuote           = 39,
    pub SDLKLeftParen       = 40,
    pub SDLKRightParen      = 41,
    pub SDLKAsterisk        = 42,
    pub SDLKPlus            = 43,
    pub SDLKComma           = 44,
    pub SDLKMinus           = 45,
    pub SDLKPeriod          = 46,
    pub SDLKSlash           = 47,
    pub SDLK0               = 48,
    pub SDLK1               = 49,
    pub SDLK2               = 50,
    pub SDLK3               = 51,
    pub SDLK4               = 52,
    pub SDLK5               = 53,
    pub SDLK6               = 54,
    pub SDLK7               = 55,
    pub SDLK8               = 56,
    pub SDLK9               = 57,
    pub SDLKColon           = 58,
    pub SDLKSemicolon       = 59,
    pub SDLKLess            = 60,
    pub SDLKEquals          = 61,
    pub SDLKGreater         = 62,
    pub SDLKQuestion        = 63,
    pub SDLKAt              = 64,
    /* 
    Skip uppercase letters
    */
    pub SDLKLeftBracket     = 91,
    pub SDLKBackslash       = 92,
    pub SDLKRightBracket    = 93,
    pub SDLKCaret           = 94,
    pub SDLKUnderscore      = 95,
    pub SDLKBackquote       = 96,
    pub SDLKa               = 97,
    pub SDLKb               = 98,
    pub SDLKc               = 99,
    pub SDLKd               = 100,
    pub SDLKe               = 101,
    pub SDLKf               = 102,
    pub SDLKg               = 103,
    pub SDLKh               = 104,
    pub SDLKi               = 105,
    pub SDLKj               = 106,
    pub SDLKk               = 107,
    pub SDLKl               = 108,
    pub SDLKm               = 109,
    pub SDLKn               = 110,
    pub SDLKo               = 111,
    pub SDLKp               = 112,
    pub SDLKq               = 113,
    pub SDLKr               = 114,
    pub SDLKs               = 115,
    pub SDLKt               = 116,
    pub SDLKu               = 117,
    pub SDLKv               = 118,
    pub SDLKw               = 119,
    pub SDLKx               = 120,
    pub SDLKy               = 121,
    pub SDLKz               = 122,
    pub SDLKDelete          = 127,
    /* End of ASCII mapped KeySyms */
    /*@}*/

    /** @name International keyboard syms */
    /*@{*/
    pub SDLKWorld0          = 160,      /* 0xA0 */
    pub SDLKWorld1          = 161,
    pub SDLKWorld2          = 162,
    pub SDLKWorld3          = 163,
    pub SDLKWorld4          = 164,
    pub SDLKWorld5          = 165,
    pub SDLKWorld6          = 166,
    pub SDLKWorld7          = 167,
    pub SDLKWorld8          = 168,
    pub SDLKWorld9          = 169,
    pub SDLKWorld10         = 170,
    pub SDLKWorld11         = 171,
    pub SDLKWorld12         = 172,
    pub SDLKWorld13         = 173,
    pub SDLKWorld14         = 174,
    pub SDLKWorld15         = 175,
    pub SDLKWorld16         = 176,
    pub SDLKWorld17         = 177,
    pub SDLKWorld18         = 178,
    pub SDLKWorld19         = 179,
    pub SDLKWorld20         = 180,
    pub SDLKWorld21         = 181,
    pub SDLKWorld22         = 182,
    pub SDLKWorld23         = 183,
    pub SDLKWorld24         = 184,
    pub SDLKWorld25         = 185,
    pub SDLKWorld26         = 186,
    pub SDLKWorld27         = 187,
    pub SDLKWorld28         = 188,
    pub SDLKWorld29         = 189,
    pub SDLKWorld30         = 190,
    pub SDLKWorld31         = 191,
    pub SDLKWorld32         = 192,
    pub SDLKWorld33         = 193,
    pub SDLKWorld34         = 194,
    pub SDLKWorld35         = 195,
    pub SDLKWorld36         = 196,
    pub SDLKWorld37         = 197,
    pub SDLKWorld38         = 198,
    pub SDLKWorld39         = 199,
    pub SDLKWorld40         = 200,
    pub SDLKWorld41         = 201,
    pub SDLKWorld42         = 202,
    pub SDLKWorld43         = 203,
    pub SDLKWorld44         = 204,
    pub SDLKWorld45         = 205,
    pub SDLKWorld46         = 206,
    pub SDLKWorld47         = 207,
    pub SDLKWorld48         = 208,
    pub SDLKWorld49         = 209,
    pub SDLKWorld50         = 210,
    pub SDLKWorld51         = 211,
    pub SDLKWorld52         = 212,
    pub SDLKWorld53         = 213,
    pub SDLKWorld54         = 214,
    pub SDLKWorld55         = 215,
    pub SDLKWorld56         = 216,
    pub SDLKWorld57         = 217,
    pub SDLKWorld58         = 218,
    pub SDLKWorld59         = 219,
    pub SDLKWorld60         = 220,
    pub SDLKWorld61         = 221,
    pub SDLKWorld62         = 222,
    pub SDLKWorld63         = 223,
    pub SDLKWorld64         = 224,
    pub SDLKWorld65         = 225,
    pub SDLKWorld66         = 226,
    pub SDLKWorld67         = 227,
    pub SDLKWorld68         = 228,
    pub SDLKWorld69         = 229,
    pub SDLKWorld70         = 230,
    pub SDLKWorld71         = 231,
    pub SDLKWorld72         = 232,
    pub SDLKWorld73         = 233,
    pub SDLKWorld74         = 234,
    pub SDLKWorld75         = 235,
    pub SDLKWorld76         = 236,
    pub SDLKWorld77         = 237,
    pub SDLKWorld78         = 238,
    pub SDLKWorld79         = 239,
    pub SDLKWorld80         = 240,
    pub SDLKWorld81         = 241,
    pub SDLKWorld82         = 242,
    pub SDLKWorld83         = 243,
    pub SDLKWorld84         = 244,
    pub SDLKWorld85         = 245,
    pub SDLKWorld86         = 246,
    pub SDLKWorld87         = 247,
    pub SDLKWorld88         = 248,
    pub SDLKWorld89         = 249,
    pub SDLKWorld90         = 250,
    pub SDLKWorld91         = 251,
    pub SDLKWorld92         = 252,
    pub SDLKWorld93         = 253,
    pub SDLKWorld94         = 254,
    pub SDLKWorld95         = 255,      /* 0xFF */
    /*@}*/

    /** @name Numeric keypad */
    /*@{*/
    pub SDLKKp0             = 256,
    pub SDLKKp1             = 257,
    pub SDLKKp2             = 258,
    pub SDLKKp3             = 259,
    pub SDLKKp4             = 260,
    pub SDLKKp5             = 261,
    pub SDLKKp6             = 262,
    pub SDLKKp7             = 263,
    pub SDLKKp8             = 264,
    pub SDLKKp9             = 265,
    pub SDLKKpPeriod        = 266,
    pub SDLKKpDivide        = 267,
    pub SDLKKpMultiply      = 268,
    pub SDLKKpMinus         = 269,
    pub SDLKKpPlus          = 270,
    pub SDLKKpEnter         = 271,
    pub SDLKKpEquals        = 272,
    /*@}*/

    /** @name Arrows + Home/End pad */
    /*@{*/
    pub SDLKUp              = 273,
    pub SDLKDown            = 274,
    pub SDLKRight           = 275,
    pub SDLKLeft            = 276,
    pub SDLKInsert          = 277,
    pub SDLKHome            = 278,
    pub SDLKEnd             = 279,
    pub SDLKPageUp          = 280,
    pub SDLKPageDown        = 281,
    /*@}*/

    /** @name Function keys */
    /*@{*/
    pub SDLKF1              = 282,
    pub SDLKF2              = 283,
    pub SDLKF3              = 284,
    pub SDLKF4              = 285,
    pub SDLKF5              = 286,
    pub SDLKF6              = 287,
    pub SDLKF7              = 288,
    pub SDLKF8              = 289,
    pub SDLKF9              = 290,
    pub SDLKF10             = 291,
    pub SDLKF11             = 292,
    pub SDLKF12             = 293,
    pub SDLKF13             = 294,
    pub SDLKF14             = 295,
    pub SDLKF15             = 296,
    /*@}*/

    /** @name Key state modifier keys */
    /*@{*/
    pub SDLKNumLock         = 300,
    pub SDLKCapsLock        = 301,
    pub SDLKScrolLock       = 302,
    pub SDLKRShift          = 303,
    pub SDLKLShift          = 304,
    pub SDLKRCtrl           = 305,
    pub SDLKLCtrl           = 306,
    pub SDLKRAlt            = 307,
    pub SDLKLAlt            = 308,
    pub SDLKRMeta           = 309,
    pub SDLKLMeta           = 310,
    pub SDLKLSuper          = 311,      /**< Left "Windows" key */
    pub SDLKRSuper          = 312,      /**< Right "Windows" key */
    pub SDLKMode            = 313,      /**< "Alt Gr" key */
    pub SDLKCompose         = 314,      /**< Multi-key compose key */
    /*@}*/

    /** @name Miscellaneous function keys */
    /*@{*/
    pub SDLKHelp            = 315,
    pub SDLKPrint           = 316,
    pub SDLKSysReq          = 317,
    pub SDLKBreak           = 318,
    pub SDLKMenu            = 319,
    pub SDLKPower           = 320,      /**< Power Macintosh power key */
    pub SDLKEuro            = 321,      /**< Some european keyboards */
    pub SDLKUndo            = 322,      /**< Atari keyboard has Undo */
    /*@}*/

    /* Add any other keys here */

    SDLKLast
}

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
