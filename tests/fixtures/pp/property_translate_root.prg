#xtranslate <w> . <p:Name,Title,f9> := <n> => SProp( <"w">, <"p"> , <n> )

PROCEDURE Main()
   oW.Title := "title"
   oW . f9 := 9
   &oW.Title := "macro"
   &oW . f9 := 10
RETURN
