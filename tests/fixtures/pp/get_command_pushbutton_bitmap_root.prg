#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> GUISEND <guimsg> SIZE X <sizex> Y <sizey> CAPOFF X <capxoff> Y <capyoff> BITMAP <bitmap> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<"var">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,<sizex>,<sizey>,<capxoff>,<capyoff>,<bitmap>,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()

PROCEDURE Main()
   @ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION "cap" MESSAGE "mes" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend() SIZE X 100 Y 100 CAPOFF X 10 Y 10 BITMAP bitmap()
RETURN
