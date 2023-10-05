; spam space space name
~$space::
{
	While GetKeyState("space", "P") {
		Send "{space}"
		Sleep 25 
	}
} 
