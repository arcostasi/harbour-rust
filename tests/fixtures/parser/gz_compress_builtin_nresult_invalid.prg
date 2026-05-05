PROCEDURE Main()
   LOCAL nResult := 1

   ? hb_gzCompress( "abc", .T., @nResult )
RETURN
