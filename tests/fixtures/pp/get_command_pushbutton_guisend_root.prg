#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> GUISEND <guimsg> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<"var">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()

PROCEDURE Main()
   @ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION "cap" MESSAGE "mes" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend()
RETURN
