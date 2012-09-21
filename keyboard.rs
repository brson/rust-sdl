use libc::c_int;

// sym corresponds to the `Key` enum, mod_ to the `Mod` enum. We should
// be using the correct type here but our enums don't have the right
// size yet
type KeySym = {
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
enum Key {
    /** @name ASCII mapped KeySyms
    *  The keyboard syms have been cleverly chosen to map to ASCII
    */
    /*@{*/
    SDLKUnknown         = 0,
    //SDLKFirst         = 0,
    SDLKBackspace       = 8,
    SDLKTab             = 9,
    SDLKClear           = 12,
    SDLKReturn          = 13,
    SDLKPause           = 19,
    SDLKEscape          = 27,
    SDLKSpace           = 32,
    SDLKExclaim         = 33,
    SDLKQuotedbl        = 34,
    SDLKHash            = 35,
    SDLKDollar          = 36,
    SDLKAmpersand       = 38,
    SDLKQuote           = 39,
    SDLKLeftParen       = 40,
    SDLKRightParen      = 41,
    SDLKAsterisk        = 42,
    SDLKPlus            = 43,
    SDLKComma           = 44,
    SDLKMinus           = 45,
    SDLKPeriod          = 46,
    SDLKSlash           = 47,
    SDLK0               = 48,
    SDLK1               = 49,
    SDLK2               = 50,
    SDLK3               = 51,
    SDLK4               = 52,
    SDLK5               = 53,
    SDLK6               = 54,
    SDLK7               = 55,
    SDLK8               = 56,
    SDLK9               = 57,
    SDLKColon           = 58,
    SDLKSemicolon       = 59,
    SDLKLess            = 60,
    SDLKEquals          = 61,
    SDLKGreater         = 62,
    SDLKQuestion        = 63,
    SDLKAt              = 64,
    /* 
    Skip uppercase letters
    */
    SDLKLeftBracket     = 91,
    SDLKBackslash       = 92,
    SDLKRightBracket    = 93,
    SDLKCaret           = 94,
    SDLKUnderscore      = 95,
    SDLKBackquote       = 96,
    SDLKa               = 97,
    SDLKb               = 98,
    SDLKc               = 99,
    SDLKd               = 100,
    SDLKe               = 101,
    SDLKf               = 102,
    SDLKg               = 103,
    SDLKh               = 104,
    SDLKi               = 105,
    SDLKj               = 106,
    SDLKk               = 107,
    SDLKl               = 108,
    SDLKm               = 109,
    SDLKn               = 110,
    SDLKo               = 111,
    SDLKp               = 112,
    SDLKq               = 113,
    SDLKr               = 114,
    SDLKs               = 115,
    SDLKt               = 116,
    SDLKu               = 117,
    SDLKv               = 118,
    SDLKw               = 119,
    SDLKx               = 120,
    SDLKy               = 121,
    SDLKz               = 122,
    SDLKDelete          = 127,
    /* End of ASCII mapped KeySyms */
    /*@}*/

    /** @name International keyboard syms */
    /*@{*/
    SDLKWorld0          = 160,      /* 0xA0 */
    SDLKWorld1          = 161,
    SDLKWorld2          = 162,
    SDLKWorld3          = 163,
    SDLKWorld4          = 164,
    SDLKWorld5          = 165,
    SDLKWorld6          = 166,
    SDLKWorld7          = 167,
    SDLKWorld8          = 168,
    SDLKWorld9          = 169,
    SDLKWorld10         = 170,
    SDLKWorld11         = 171,
    SDLKWorld12         = 172,
    SDLKWorld13         = 173,
    SDLKWorld14         = 174,
    SDLKWorld15         = 175,
    SDLKWorld16         = 176,
    SDLKWorld17         = 177,
    SDLKWorld18         = 178,
    SDLKWorld19         = 179,
    SDLKWorld20         = 180,
    SDLKWorld21         = 181,
    SDLKWorld22         = 182,
    SDLKWorld23         = 183,
    SDLKWorld24         = 184,
    SDLKWorld25         = 185,
    SDLKWorld26         = 186,
    SDLKWorld27         = 187,
    SDLKWorld28         = 188,
    SDLKWorld29         = 189,
    SDLKWorld30         = 190,
    SDLKWorld31         = 191,
    SDLKWorld32         = 192,
    SDLKWorld33         = 193,
    SDLKWorld34         = 194,
    SDLKWorld35         = 195,
    SDLKWorld36         = 196,
    SDLKWorld37         = 197,
    SDLKWorld38         = 198,
    SDLKWorld39         = 199,
    SDLKWorld40         = 200,
    SDLKWorld41         = 201,
    SDLKWorld42         = 202,
    SDLKWorld43         = 203,
    SDLKWorld44         = 204,
    SDLKWorld45         = 205,
    SDLKWorld46         = 206,
    SDLKWorld47         = 207,
    SDLKWorld48         = 208,
    SDLKWorld49         = 209,
    SDLKWorld50         = 210,
    SDLKWorld51         = 211,
    SDLKWorld52         = 212,
    SDLKWorld53         = 213,
    SDLKWorld54         = 214,
    SDLKWorld55         = 215,
    SDLKWorld56         = 216,
    SDLKWorld57         = 217,
    SDLKWorld58         = 218,
    SDLKWorld59         = 219,
    SDLKWorld60         = 220,
    SDLKWorld61         = 221,
    SDLKWorld62         = 222,
    SDLKWorld63         = 223,
    SDLKWorld64         = 224,
    SDLKWorld65         = 225,
    SDLKWorld66         = 226,
    SDLKWorld67         = 227,
    SDLKWorld68         = 228,
    SDLKWorld69         = 229,
    SDLKWorld70         = 230,
    SDLKWorld71         = 231,
    SDLKWorld72         = 232,
    SDLKWorld73         = 233,
    SDLKWorld74         = 234,
    SDLKWorld75         = 235,
    SDLKWorld76         = 236,
    SDLKWorld77         = 237,
    SDLKWorld78         = 238,
    SDLKWorld79         = 239,
    SDLKWorld80         = 240,
    SDLKWorld81         = 241,
    SDLKWorld82         = 242,
    SDLKWorld83         = 243,
    SDLKWorld84         = 244,
    SDLKWorld85         = 245,
    SDLKWorld86         = 246,
    SDLKWorld87         = 247,
    SDLKWorld88         = 248,
    SDLKWorld89         = 249,
    SDLKWorld90         = 250,
    SDLKWorld91         = 251,
    SDLKWorld92         = 252,
    SDLKWorld93         = 253,
    SDLKWorld94         = 254,
    SDLKWorld95         = 255,      /* 0xFF */
    /*@}*/

    /** @name Numeric keypad */
    /*@{*/
    SDLKKp0             = 256,
    SDLKKp1             = 257,
    SDLKKp2             = 258,
    SDLKKp3             = 259,
    SDLKKp4             = 260,
    SDLKKp5             = 261,
    SDLKKp6             = 262,
    SDLKKp7             = 263,
    SDLKKp8             = 264,
    SDLKKp9             = 265,
    SDLKKpPeriod        = 266,
    SDLKKpDivide        = 267,
    SDLKKpMultiply      = 268,
    SDLKKpMinus         = 269,
    SDLKKpPlus          = 270,
    SDLKKpEnter         = 271,
    SDLKKpEquals        = 272,
    /*@}*/

    /** @name Arrows + Home/End pad */
    /*@{*/
    SDLKUp              = 273,
    SDLKDown            = 274,
    SDLKRight           = 275,
    SDLKLeft            = 276,
    SDLKInsert          = 277,
    SDLKHome            = 278,
    SDLKEnd             = 279,
    SDLKPageUp          = 280,
    SDLKPageDown        = 281,
    /*@}*/

    /** @name Function keys */
    /*@{*/
    SDLKF1              = 282,
    SDLKF2              = 283,
    SDLKF3              = 284,
    SDLKF4              = 285,
    SDLKF5              = 286,
    SDLKF6              = 287,
    SDLKF7              = 288,
    SDLKF8              = 289,
    SDLKF9              = 290,
    SDLKF10             = 291,
    SDLKF11             = 292,
    SDLKF12             = 293,
    SDLKF13             = 294,
    SDLKF14             = 295,
    SDLKF15             = 296,
    /*@}*/

    /** @name Key state modifier keys */
    /*@{*/
    SDLKNumLock         = 300,
    SDLKCapsLock        = 301,
    SDLKScrolLock       = 302,
    SDLKRShift          = 303,
    SDLKLShift          = 304,
    SDLKRCtrl           = 305,
    SDLKLCtrl           = 306,
    SDLKRAlt            = 307,
    SDLKLAlt            = 308,
    SDLKRMeta           = 309,
    SDLKLMeta           = 310,
    SDLKLSuper          = 311,      /**< Left "Windows" key */
    SDLKRSuper          = 312,      /**< Right "Windows" key */
    SDLKMode            = 313,      /**< "Alt Gr" key */
    SDLKCompose         = 314,      /**< Multi-key compose key */
    /*@}*/

    /** @name Miscellaneous function keys */
    /*@{*/
    SDLKHelp            = 315,
    SDLKPrint           = 316,
    SDLKSysReq          = 317,
    SDLKBreak           = 318,
    SDLKMenu            = 319,
    SDLKPower           = 320,      /**< Power Macintosh power key */
    SDLKEuro            = 321,      /**< Some european keyboards */
    SDLKUndo            = 322,      /**< Atari keyboard has Undo */
    /*@}*/

    /* Add any other keys here */

    SDLKLast
}

/** Enumeration of valid key mods (possibly OR'd together) */
enum Mod {
    KMODNone            = 0x0000,
    KMODLShift          = 0x0001,
    KMODRShift          = 0x0002,
    KMODLCtrl           = 0x0040,
    KMODRCtrl           = 0x0080,
    KMODLAlt            = 0x0100,
    KMODRAlt            = 0x0200,
    KMODLMeta           = 0x0400,
    KMODRMeta           = 0x0800,
    KMODNum             = 0x1000,
    KMODCaps            = 0x2000,
    KMODMode            = 0x4000,
    KMODReserved        = 0x8000
}
