PROCEDURE Main()
   LOCAL cOut := ""

   ? hb_processRun( "echo hbrust", NIL, @cOut )
   ? Left( cOut, 6 )
RETURN
