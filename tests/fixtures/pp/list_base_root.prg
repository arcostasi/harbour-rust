#command LIST [<v,...>] [<off:OFF>] [<prn:TO PRINTER>] [TO FILE <(f)>] ;
              [FOR <for>] [WHILE <while>] [NEXT <next>] ;
              [RECORD <rec>] [<rest:REST>] [ALL] => ;
         __dbList( <.off.>, { <{v}> }, .t., ;
                   <{for}>, <{while}>, <next>, <rec>, <.rest.>, <.prn.>, <(f)> )

PROCEDURE Main()
   LIST
RETURN
