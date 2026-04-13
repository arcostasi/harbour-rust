#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<"var">,NIL,<{valid}>, ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()

PROCEDURE Main()
   @ 4,1 GET a PUSHBUTTON VALID valid()
RETURN
