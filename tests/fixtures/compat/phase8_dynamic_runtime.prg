PROCEDURE Main()
   PUBLIC counter := 10
   PUBLIC total := 10
   PRIVATE counter := 4

   ? Eval( {|x, y| x + y }, 2, 3 )
   ? Eval( {|| counter + 1 } )
   ? ValType( {|x, y| x + y } )
   ? Empty( {|x, y| x + y } )
RETURN
