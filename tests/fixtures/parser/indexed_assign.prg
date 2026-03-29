// Indexed assignment baseline

PROCEDURE Main()

   LOCAL matrix := { { 10, 20 }, { 30, 40 } }

   matrix[ 2 ][ 1 ] := 99
   ? matrix[ 2 ][ 1 ]

   RETURN
