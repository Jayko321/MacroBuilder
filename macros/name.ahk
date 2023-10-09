; spam space space name --delay=225
~$space::
{
	While GetKeyState("space", "P") {
		Send "{space}"
		Sleep 225 
	}
} 
