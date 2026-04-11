#command INDEX ON <key> TO <(file)> [<u: UNIQUE>] => ;
            dbCreateIndex( <(file)>, <"key">, <{key}>, iif( <.u.>, .t., NIL ) )
PROCEDURE Main()
   index on LEFT(   f1  ,  10   )      to _tst
RETURN
