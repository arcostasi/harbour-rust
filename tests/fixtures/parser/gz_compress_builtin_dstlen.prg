PROCEDURE Main()
   LOCAL nResult := 1
   LOCAL compressed := hb_gzCompress( "abc", 26, @nResult )

   ? ValType( compressed )
   ? Len( compressed )
   ? nResult

   compressed := hb_gzCompress( "abc", 25, @nResult )
   ? ValType( compressed )
   ? nResult

   ? ValType( hb_gzCompress( "abc", 26 ) )
   ? ValType( hb_gzCompress( "abc", 25 ) )
RETURN
