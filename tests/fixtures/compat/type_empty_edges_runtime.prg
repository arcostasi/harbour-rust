PROCEDURE Main()
   LOCAL empty_items := {}
   LOCAL filled_items := { 0 }

   ? ValType( .F. )
   ? ValType( "" )
   ? ValType( empty_items )
   ? Empty( " " + Chr( 13 ) + Chr( 9 ) )
   ? Empty( "  A" )
   ? Empty( empty_items )
   ? Empty( filled_items )
RETURN
