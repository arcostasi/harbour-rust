PROCEDURE Main()
   LOCAL empty_items := {}
   LOCAL filled_items := { 0 }

   ? Empty()
   ? Empty( NIL )
   ? Empty( .F. )
   ? Empty( .T. )
   ? Empty( 0 )
   ? Empty( 10 )
   ? Empty( "" )
   ? Empty( "  " )
   ? Empty( "A" )
   ? Empty( empty_items )
   ? Empty( filled_items )
RETURN
