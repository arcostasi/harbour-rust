PROCEDURE Main()
   LOCAL nResult := 1
   LOCAL cBuffer := Space( 26 )
   LOCAL compressed := hb_gzCompress( "abc", @cBuffer, @nResult )

   ? ValType( compressed )
   ? Len( compressed )
   ? nResult
   ? ValType( cBuffer )
   ? Len( cBuffer )
RETURN
