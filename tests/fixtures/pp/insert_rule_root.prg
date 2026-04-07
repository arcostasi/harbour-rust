#xcommand INSERT INTO <table> ( <uField1> [, <uFieldN> ] ) VALUES ( <uVal1> [, <uValN> ] ) => ;
if <table>->( dbappend() ) ;;
 replace <table>-><uField1> with <uVal1> ;;
 [ replace <table>-><uFieldN> with <uValN> ; ] ;
 <table>->( dbunlock() ) ;;
endif
#xcommand INSERT2 INTO <table> ( <uField1> [, <uFieldN> ] ) VALUES ( <uVal1> [, <uValN> ] ) => ;
if <table>->( dbappend() ) ;;
 <table>-><uField1> := <uVal1> ;;
 [ <table>-><uFieldN> := <uValN> ; ] ;
 <table>->( dbunlock() ) ;;
endif
PROCEDURE Main()
   insert into test ( FIRST, LAST, STREET ) values ( "first", "last", "street" )
   insert2 into test ( FIRST, LAST, STREET ) ;
      values ( "first", "last", "street" )
RETURN
