#command @ <row>, <col> GET <var> PUSHBUTTON COLOR <color> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<"var">,NIL,, ) ) ; ATail(GetList):Control := _PushButt_(,,<color>,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()

PROCEDURE Main()
   @ 4,1 GET a PUSHBUTTON COLOR "W/N"
RETURN
