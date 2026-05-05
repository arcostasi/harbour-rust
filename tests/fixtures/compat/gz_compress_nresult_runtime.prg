PROCEDURE Main()
   LOCAL nResult := 1
   LOCAL compressed := hb_gzCompress( "abc", NIL, @nResult )
   LOCAL cBuffer := Space( 26 )
   LOCAL cSmallBuffer := Space( 25 )

   ? ValType( compressed )
   ? Len( compressed )
   ? nResult

   compressed := hb_gzCompress( "", NIL, @nResult )
   ? Len( compressed )
   ? nResult

   compressed := hb_gzCompress( "abc", @cBuffer, @nResult )
   ? ValType( compressed )
   ? Len( cBuffer )
   ? nResult

   compressed := hb_gzCompress( "abc", @cSmallBuffer, @nResult )
   ? ValType( compressed )
   ? Len( cSmallBuffer )
   ? nResult
RETURN
