PROCEDURE Main()
   LOCAL nResult := 1
   LOCAL compressed := hb_gzCompress( "abc", NIL, @nResult )

   ? ValType( compressed )
   ? Len( compressed )
   ? nResult

   compressed := hb_gzCompress( "", NIL, @nResult )
   ? Len( compressed )
   ? nResult
RETURN
