#command MYCOMMAND2 [<myList,...>]
   [MYCLAUSE <myVal>] [MYOTHER <myOther>] => MyFunction( {<myList>}, <myVal>, <myOther> )
PROCEDURE Main()
   MYCOMMAND2 MYCLAUSE 322 "Hello" MYOTHER 1
RETURN
