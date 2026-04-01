PROCEDURE Main()
   LOCAL names := { "Alice", "Bob" }

   ? ValType()
   ? ValType( NIL )
   ? ValType( .T. )
   ? ValType( 10 )
   ? ValType( 10.5 )
   ? ValType( "abc" )
   ? ValType( names )
RETURN
