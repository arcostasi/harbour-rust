PROCEDURE Main()
   LOCAL empty_items := {}
   LOCAL filled_items := { 0 }

   ? Empty()
   ? Empty( NIL )
   ? Empty( "" )
   ? Empty( "A" )
   ? Empty( 0 )
   ? Empty( 10 )
   ? Empty( .F. )
   ? Empty( .T. )
   ? Empty( empty_items )
   ? Empty( filled_items )
RETURN
