#xcommand INSERT2 INTO <table> ( <uField1> [, <uFieldN> ] ) VALUES ( <uVal1> [, <uValN> ] ) =>
if <table>->( dbappend() ) ;
 <table>-><uField1> := <uVal1> ;
 [ <table>-><uFieldN> := <uValN> ; ]
 <table>->( dbunlock() ) ;
endif
#command MYCOMMAND2 [<mylist,...>] [MYCLAUSE <myval>] [ALL] =>
   MyFunction( {<mylist>} [, <myval>] )
#command MYCOMMAND3 [<mylist,...>] [MYCLAUSE <myval>] [<all:ALL>] =>
   MyFunction( {<mylist>} [, <myval>] [,<.all.>] )
PROCEDURE Main()
   insert2 into test ( FIRST, LAST, STREET ) ;
      values ( "first", "last", "street" )
   MYCOMMAND2 MYCLAUSE 321 "HELLO"
   MYCOMMAND2 ALL MYCLAUSE 321 "HELLO"
   MYCOMMAND2 MYCLAUSE 321 ALL "HELLO"
   MYCOMMAND2 MYCLAUSE 321 "HELLO" ALL
   MYCOMMAND3 ALL MYCLAUSE 321 "HELLO","WORLD"
   MYCOMMAND3 MYCLAUSE 321 "HELLO"
RETURN
