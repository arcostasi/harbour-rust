PROCEDURE Main()
   ? Trim( " " + Chr( 0 ) + " UA  " )
   ? Trim( " " + Chr( 9 ) + " UA  " )
   ? LTrim( " " + Chr( 0 ) + " UA  " )
   ? LTrim( " " + Chr( 9 ) + "U" + Chr( 9 ) )
   ? LTrim( Chr( 10 ) + "U" + Chr( 10 ) )
   ? RTrim( "A" + Chr( 10 ) )
   ? Trim( "  " + Chr( 0 ) + "ABC" + Chr( 0 ) + "  " )
RETURN
