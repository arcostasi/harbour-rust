PROCEDURE Main()
   LOCAL compressed := hb_gzCompress( "abc" )

   ? ValType( compressed )
   ? Len( compressed )
   ? Len( hb_gzCompress( "" ) )
RETURN
