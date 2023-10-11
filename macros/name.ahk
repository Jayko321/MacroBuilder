; spam f f name --delay=10 --target=Notepad
~$f::
{
	While GetKeyState("f", "P") & WinActive("ahk_class Notepad") {
		Send "{f}"
		Sleep 10 
	}
} 
