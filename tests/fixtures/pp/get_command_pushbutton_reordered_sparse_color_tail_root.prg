#command @ <row>, <col> GET <var> PUSHBUTTON SIZE X <sizex> Y <sizey> BMPOFF X <bmpxoff> Y <bmpyoff> VALID <valid> GUISEND <guimsg> WHEN <when> MESSAGE <message> COLOR <color> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<"var">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(,<message>,<color>,,,,<sizex>,<sizey>,,,,<bmpxoff>,<bmpyoff> ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()

PROCEDURE Main()
   @ 4,1 GET a PUSHBUTTON SIZE X 100 Y 100 BMPOFF X 2 Y 2 VALID valid() GUISEND guisend() WHEN when() MESSAGE "mes" COLOR "W/N"
RETURN
