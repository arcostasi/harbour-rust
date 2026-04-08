#xtranslate XTRANS(<x>( => normal( <(x)> )
#xtranslate XTRANS(<x:&>( => macro( <(x)> )
PROCEDURE Main()
   XTRANS( cVar (
   XTRANS( &cVar (
   XTRANS( &cVar+1 (
   XTRANS( &(cVar) (
RETURN
