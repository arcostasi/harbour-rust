#translate MTRANSLATE <x:&> => macro_t( <(x)> )
PROCEDURE Main()
   MTRANSLATE &cVar
   MTRANSLATE &cVar.
   MTRANSLATE &(cVar)
   MTRANSLATE & (cVar)
   MTRANSLATE &cVar&cVar
   MTRANSLATE &cVar+1
   MTRANSLATE &cVar.+1
   MTRANSLATE &cVar .AND. .T.
   MTRANSLATE &(cVar) +1
RETURN