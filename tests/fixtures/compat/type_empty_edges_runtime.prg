PROCEDURE Main()
   LOCAL empty_items := {}
   LOCAL filled_items := { 0 }

   ? ValType( .F. )
   ? ValType( "" )
   ? ValType( empty_items )
   ? ValType( {|x| x } )
   ? Empty( " " + Chr( 13 ) + Chr( 9 ) )
   ? Empty( "  A" )
   ? Empty( empty_items )
   ? Empty( filled_items )
   ? Empty( {|x| x } )
RETURN
