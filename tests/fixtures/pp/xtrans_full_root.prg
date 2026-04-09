#xtranslate XTRANS(<x>( => normal( <(x)> )
#xtranslate XTRANS(<x:&>( => macro( <(x)> )
PROCEDURE Main()
   XTRANS( cVar (
   XTRANS( &cVar (
   XTRANS( &cVar+1 (
   XTRANS( &cVar. (
   XTRANS( &cVar&cVar (
   XTRANS( &cVar.&cVar (
   XTRANS( &cVar.&cVar. (
   XTRANS( (&cVar.) (
   XTRANS( &(cVar) (
   XTRANS( &cVar[3] (
   XTRANS( &cVar.  [3] (
   XTRANS( &(cVar  [3],&cvar) (
   XTRANS( (&cVar.  [3],&cvar) (
   XTRANS( &cVar.1+5 (
   XTRANS( &cVar .AND. cVar (
   XTRANS( &cVar. .AND. cVar (
RETURN
