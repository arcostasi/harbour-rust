#command @ <row>, <col> GET <var> PUSHBUTTON SIZE X <sizex> Y <sizey> BMPOFF X <bmpxoff> Y <bmpyoff> VALID <valid> GUISEND <guimsg> WHEN <when> MESSAGE <message> COLOR <color> CAPOFF X <capxoff> Y <capyoff> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> BITMAP <bitmap> CAPTION <caption> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<"var">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,<sizex>,<sizey>,<capxoff>,<capyoff>,<bitmap>,<bmpxoff>,<bmpyoff> ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()

PROCEDURE Main()
   @ 4,1 GET a PUSHBUTTON SIZE X 100 Y 100 BMPOFF X 2 Y 2 VALID valid() GUISEND guisend() WHEN when() MESSAGE "mes" COLOR "W/N" CAPOFF X 10 Y 10 FOCUS focus() STATE state() STYLE style() SEND send() BITMAP bitmap() CAPTION "cap"
RETURN
