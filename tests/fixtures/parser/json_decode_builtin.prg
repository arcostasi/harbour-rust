PROCEDURE Main()
   LOCAL decoded := hb_JsonDecode( '{"ok":true,"items":[1,null,"x"]}' )

   ? ValType( decoded )
   ? Len( decoded )
   ? decoded[ 1 ][ 1 ]
   ? decoded[ 1 ][ 2 ]
   ? decoded[ 2 ][ 1 ]
   ? decoded[ 2 ][ 2 ][ 1 ]
   ? ValType( decoded[ 2 ][ 2 ][ 2 ] )
   ? decoded[ 2 ][ 2 ][ 3 ]
RETURN
