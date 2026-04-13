#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<"var">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()

PROCEDURE Main()
   @ 4,1 GET a PUSHBUTTON VALID valid() WHEN when()
RETURN
