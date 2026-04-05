PROCEDURE Main()
   LOCAL negFive := 0 - 5
   LOCAL negEight := 0 - 8
   LOCAL negTen := 0 - 10
   LOCAL negTenPointFive := 0 - 10.5

   ? Str( 10 )
   ? Str( 0 )
   ? Str( 10.5 )
   ? Str( 10, 5 )
   ? Str( 10, negFive )
   ? Str( 10.6, 5 )
   ? Str( 10.5, negFive )
   ? Str( negTenPointFive, negFive )
   ? Str( negTen, negFive )
   ? Str( 2, 5, 2 )
   ? Str( 10.5, 5, 0 )
   ? Str( negTenPointFive, 5, 0 )
   ? Str( 3.125, 8, 2 )
   ? Str( 100000, negEight )
   ? Str( 100000, 5 )
RETURN
