PROCEDURE Main()
   ? SubStr( "abcdef", 0 )
   ? SubStr( "abcdef", -10, 1 )
   ? SubStr( "abcdef", -10, 15 )
   ? SubStr( "abcdef", 2, 0 )
   ? SubStr( "ab" + Chr( 0 ) + "def", 2, 3 )
   ? SubStr( "abc" + Chr( 0 ) + "def", 4, 1 )
   ? SubStr( "abcdef", 1, "a" )
RETURN
