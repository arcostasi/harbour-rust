#define clas( x )   (x)
#xtranslate ( <name>{ [<p,...>] } => (<name>():New(<p>)

PROCEDURE Main()
   ? clas( TEST{ 1,2,3} )
   ? clas( a+3{ 11,2,3} )
   ? clas( a(){ 11,2,3} )
RETURN
