// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

pub const ALLEGRO_KEY_A: c_uint = 1;
pub const ALLEGRO_KEY_B: c_uint = 2;
pub const ALLEGRO_KEY_C: c_uint = 3;
pub const ALLEGRO_KEY_D: c_uint = 4;
pub const ALLEGRO_KEY_E: c_uint = 5;
pub const ALLEGRO_KEY_F: c_uint = 6;
pub const ALLEGRO_KEY_G: c_uint = 7;
pub const ALLEGRO_KEY_H: c_uint = 8;
pub const ALLEGRO_KEY_I: c_uint = 9;
pub const ALLEGRO_KEY_J: c_uint = 10;
pub const ALLEGRO_KEY_K: c_uint = 11;
pub const ALLEGRO_KEY_L: c_uint = 12;
pub const ALLEGRO_KEY_M: c_uint = 13;
pub const ALLEGRO_KEY_N: c_uint = 14;
pub const ALLEGRO_KEY_O: c_uint = 15;
pub const ALLEGRO_KEY_P: c_uint = 16;
pub const ALLEGRO_KEY_Q: c_uint = 17;
pub const ALLEGRO_KEY_R: c_uint = 18;
pub const ALLEGRO_KEY_S: c_uint = 19;
pub const ALLEGRO_KEY_T: c_uint = 20;
pub const ALLEGRO_KEY_U: c_uint = 21;
pub const ALLEGRO_KEY_V: c_uint = 22;
pub const ALLEGRO_KEY_W: c_uint = 23;
pub const ALLEGRO_KEY_X: c_uint = 24;
pub const ALLEGRO_KEY_Y: c_uint = 25;
pub const ALLEGRO_KEY_Z: c_uint = 26;
pub const ALLEGRO_KEY_0: c_uint = 27;
pub const ALLEGRO_KEY_1: c_uint = 28;
pub const ALLEGRO_KEY_2: c_uint = 29;
pub const ALLEGRO_KEY_3: c_uint = 30;
pub const ALLEGRO_KEY_4: c_uint = 31;
pub const ALLEGRO_KEY_5: c_uint = 32;
pub const ALLEGRO_KEY_6: c_uint = 33;
pub const ALLEGRO_KEY_7: c_uint = 34;
pub const ALLEGRO_KEY_8: c_uint = 35;
pub const ALLEGRO_KEY_9: c_uint = 36;
pub const ALLEGRO_KEY_PAD_0: c_uint = 37;
pub const ALLEGRO_KEY_PAD_1: c_uint = 38;
pub const ALLEGRO_KEY_PAD_2: c_uint = 39;
pub const ALLEGRO_KEY_PAD_3: c_uint = 40;
pub const ALLEGRO_KEY_PAD_4: c_uint = 41;
pub const ALLEGRO_KEY_PAD_5: c_uint = 42;
pub const ALLEGRO_KEY_PAD_6: c_uint = 43;
pub const ALLEGRO_KEY_PAD_7: c_uint = 44;
pub const ALLEGRO_KEY_PAD_8: c_uint = 45;
pub const ALLEGRO_KEY_PAD_9: c_uint = 46;
pub const ALLEGRO_KEY_F1: c_uint = 47;
pub const ALLEGRO_KEY_F2: c_uint = 48;
pub const ALLEGRO_KEY_F3: c_uint = 49;
pub const ALLEGRO_KEY_F4: c_uint = 50;
pub const ALLEGRO_KEY_F5: c_uint = 51;
pub const ALLEGRO_KEY_F6: c_uint = 52;
pub const ALLEGRO_KEY_F7: c_uint = 53;
pub const ALLEGRO_KEY_F8: c_uint = 54;
pub const ALLEGRO_KEY_F9: c_uint = 55;
pub const ALLEGRO_KEY_F10: c_uint = 56;
pub const ALLEGRO_KEY_F11: c_uint = 57;
pub const ALLEGRO_KEY_F12: c_uint = 58;
pub const ALLEGRO_KEY_ESCAPE: c_uint = 59;
pub const ALLEGRO_KEY_TILDE: c_uint = 60;
pub const ALLEGRO_KEY_MINUS: c_uint = 61;
pub const ALLEGRO_KEY_EQUALS: c_uint = 62;
pub const ALLEGRO_KEY_BACKSPACE: c_uint = 63;
pub const ALLEGRO_KEY_TAB: c_uint = 64;
pub const ALLEGRO_KEY_OPENBRACE: c_uint = 65;
pub const ALLEGRO_KEY_CLOSEBRACE: c_uint = 66;
pub const ALLEGRO_KEY_ENTER: c_uint = 67;
pub const ALLEGRO_KEY_SEMICOLON: c_uint = 68;
pub const ALLEGRO_KEY_QUOTE: c_uint = 69;
pub const ALLEGRO_KEY_BACKSLASH: c_uint = 70;
pub const ALLEGRO_KEY_BACKSLASH2: c_uint = 71;
pub const ALLEGRO_KEY_COMMA: c_uint = 72;
pub const ALLEGRO_KEY_FULLSTOP: c_uint = 73;
pub const ALLEGRO_KEY_SLASH: c_uint = 74;
pub const ALLEGRO_KEY_SPACE: c_uint = 75;
pub const ALLEGRO_KEY_INSERT: c_uint = 76;
pub const ALLEGRO_KEY_DELETE: c_uint = 77;
pub const ALLEGRO_KEY_HOME: c_uint = 78;
pub const ALLEGRO_KEY_END: c_uint = 79;
pub const ALLEGRO_KEY_PGUP: c_uint = 80;
pub const ALLEGRO_KEY_PGDN: c_uint = 81;
pub const ALLEGRO_KEY_LEFT: c_uint = 82;
pub const ALLEGRO_KEY_RIGHT: c_uint = 83;
pub const ALLEGRO_KEY_UP: c_uint = 84;
pub const ALLEGRO_KEY_DOWN: c_uint = 85;
pub const ALLEGRO_KEY_PAD_SLASH: c_uint = 86;
pub const ALLEGRO_KEY_PAD_ASTERISK: c_uint = 87;
pub const ALLEGRO_KEY_PAD_MINUS: c_uint = 88;
pub const ALLEGRO_KEY_PAD_PLUS: c_uint = 89;
pub const ALLEGRO_KEY_PAD_DELETE: c_uint = 90;
pub const ALLEGRO_KEY_PAD_ENTER: c_uint = 91;
pub const ALLEGRO_KEY_PRINTSCREEN: c_uint = 92;
pub const ALLEGRO_KEY_PAUSE: c_uint = 93;
pub const ALLEGRO_KEY_ABNT_C1: c_uint = 94;
pub const ALLEGRO_KEY_YEN: c_uint = 95;
pub const ALLEGRO_KEY_KANA: c_uint = 96;
pub const ALLEGRO_KEY_CONVERT: c_uint = 97;
pub const ALLEGRO_KEY_NOCONVERT: c_uint = 98;
pub const ALLEGRO_KEY_AT: c_uint = 99;
pub const ALLEGRO_KEY_CIRCUMFLEX: c_uint = 100;
pub const ALLEGRO_KEY_COLON2: c_uint = 101;
pub const ALLEGRO_KEY_KANJI: c_uint = 102;
pub const ALLEGRO_KEY_PAD_EQUALS: c_uint = 103;
pub const ALLEGRO_KEY_BACKQUOTE: c_uint = 104;
pub const ALLEGRO_KEY_SEMICOLON2: c_uint = 105;
pub const ALLEGRO_KEY_COMMAND: c_uint = 106;
pub const ALLEGRO_KEY_UNKNOWN: c_uint = 107;

pub const ALLEGRO_KEY_MODIFIERS: c_uint = 215;
pub const ALLEGRO_KEY_LSHIFT: c_uint = 215;
pub const ALLEGRO_KEY_RSHIFT: c_uint = 216;
pub const ALLEGRO_KEY_LCTRL: c_uint = 217;
pub const ALLEGRO_KEY_RCTRL: c_uint = 218;
pub const ALLEGRO_KEY_ALT: c_uint = 219;
pub const ALLEGRO_KEY_ALTGR: c_uint = 220;
pub const ALLEGRO_KEY_LWIN: c_uint = 221;
pub const ALLEGRO_KEY_RWIN: c_uint = 222;
pub const ALLEGRO_KEY_MENU: c_uint = 223;
pub const ALLEGRO_KEY_SCROLLLOCK: c_uint = 224;
pub const ALLEGRO_KEY_NUMLOCK: c_uint = 225;
pub const ALLEGRO_KEY_CAPSLOCK: c_uint = 226;
pub const ALLEGRO_KEY_MAX: c_uint = 227;

pub const ALLEGRO_KEYMOD_SHIFT: c_uint = 1;
pub const ALLEGRO_KEYMOD_CTRL: c_uint = 2;
pub const ALLEGRO_KEYMOD_ALT: c_uint = 4;
pub const ALLEGRO_KEYMOD_LWIN: c_uint = 8;
pub const ALLEGRO_KEYMOD_RWIN: c_uint = 16;
pub const ALLEGRO_KEYMOD_MENU: c_uint = 32;
pub const ALLEGRO_KEYMOD_ALTGR: c_uint = 64;
pub const ALLEGRO_KEYMOD_COMMAND: c_uint = 128;
pub const ALLEGRO_KEYMOD_SCROLLLOCK: c_uint = 256;
pub const ALLEGRO_KEYMOD_NUMLOCK: c_uint = 512;
pub const ALLEGRO_KEYMOD_CAPSLOCK: c_uint = 1024;
pub const ALLEGRO_KEYMOD_INALTSEQ: c_uint = 2048;
pub const ALLEGRO_KEYMOD_ACCENT1: c_uint = 4096;
pub const ALLEGRO_KEYMOD_ACCENT2: c_uint = 8192;
pub const ALLEGRO_KEYMOD_ACCENT3: c_uint = 16384;
pub const ALLEGRO_KEYMOD_ACCENT4: c_uint = 32768;
