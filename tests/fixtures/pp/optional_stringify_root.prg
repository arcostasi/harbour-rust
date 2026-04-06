#command ZZZ [<v>] => QOUT( [ <v>\[1\] ] )
#command _DUMB_M(<z>) => dm( #<z> )
#command MYCOMMAND [<mylist,...>] [MYCLAUSE <myval>] => ;
   MyFunction( {<mylist>} [, <myval>] )
PROCEDURE Main()
   ZZZ
   ZZZ a
   _DUMB_M(a)
   MYCOMMAND MYCLAUSE 321 "HELLO"
   MYCOMMAND "HELLO","all" MYCLAUSE 321
RETURN
