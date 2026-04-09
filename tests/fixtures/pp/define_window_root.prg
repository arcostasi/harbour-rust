#xcommand DECLARE WINDOW <w> ;
=>;
#xtranslate <w> . <p:Name,Title,f1,f2,f3,f4,f5,f6,f7,f8,f9> := <n> => SProp( <"w">, <"p"> , <n> )
#xcommand DEFINE WINDOW <w> [ON INIT <IProc>] =>;
      DECLARE WINDOW <w>  ; _DW( <"w">, <{IProc}> )

PROCEDURE Main()
   DEFINE WINDOW &oW
   DEFINE WINDOW &oW ON INIT &oW.Title:= "My title"
   &oW.Title := "title"
   &oW.f9 := 9
RETURN
