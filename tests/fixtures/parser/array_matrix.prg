PROCEDURE Main()

   LOCAL matrix := { { 1, 2 }, { 3, 4 } }

   ? matrix[ 1 ][ 2 ]
   matrix[ 2 ][ 1 ] := 99
   ? matrix[ 2 ][ 1 ]
   ? Len( matrix[ 1 ] )

   RETURN
