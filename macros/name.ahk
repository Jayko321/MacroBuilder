; spam f+a l+b name
~$l::
~$b::
{
	While GetKeyState("l", "P") & GetKeyState("b", "P") {
		Send "fa"
		Sleep 25 
	}
} 
