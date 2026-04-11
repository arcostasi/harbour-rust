#command SET FILTER TO <exp>     => dbSetFilter( <{exp}>, <"exp"> )
#command SET FILTER TO <x:&>     => if ( Empty( <(x)> ) ) ; dbClearFilter() ;; else ; dbSetFilter( <{x}>, <(x)> ) ; end

PROCEDURE Main()
   SET FILTER TO &cVar.
   SET FILTER TO &(cVar .AND. &cVar)
   SET FILTER TO &cVar. .AND. cVar
RETURN
