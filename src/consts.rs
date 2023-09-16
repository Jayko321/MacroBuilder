pub const NOT_ENOUGH_ARGUMENTS: &str = "Not enough arguments, type help for help:)";

pub const KEYS: &'static [&'static str] = &[
    "esc", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12", "prtScr",
    "scrLk", "pause", "`", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "bsp",
    "ins", "home", "pgUp", "numLock", "numSlash", "numStar", "numMinus", "tab", "q", "w", "e", "r", "t", "y",
    "u", "i", "o", "p", "[", "]", "\\", "del", "end", "pgDn", "num7", "num8", "num9", "numPlus",
    "caps", "a", "s", "d", "f", "g", "h", "j", "k", "L", ";", "'", "enter", "num4", "num5", "num6",
    "LShift", "z", "x", "c", "v", "B", "n", "m", ",", ".", "/", "RShift", "up", "num1", "num2",
    "num3", "numEnter", "lCtrl", "lMeta", "lAlt", "space", "rAlt", "rMeta", "rCtrl", "left",
    "down", "right", "num0", "numDot",
];

pub const AHK_HEADER: &str = "#NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode Input  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir %A_ScriptDir%  ; Ensures a consistent starting directory.

";
