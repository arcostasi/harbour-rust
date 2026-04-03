PROCEDURE Main()
   LOCAL values := { 10, 20, 30 }
   LOCAL words := { "HELLO", "", "WORLD" }

   ? AScan( values, 20 )
   ? AScan( values, 0 )
   ? AScan( words, "HELL" )

   ? AIns( values, 2 )
   ? values[ 1 ]
   ? ValType( values[ 2 ] )
   ? values[ 3 ]

   ? ADel( values, 1 )
   ? ValType( values[ 1 ] )
   ? values[ 2 ]
   ? ValType( values[ 3 ] )
RETURN
