PROCEDURE Main()
   LOCAL names := { "Alice", "Bob" }
   LOCAL block := {|x| x }

   ? ValType()
   ? ValType( NIL )
   ? ValType( .T. )
   ? ValType( 10 )
   ? ValType( 10.5 )
   ? ValType( "abc" )
   ? ValType( names )
   ? ValType( block )
RETURN
