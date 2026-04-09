#xtranslate MXCALL <x:&> => (<x>)
PROCEDURE Main()
   MXCALL &cVar()
   MXCALL &cVar++
   (MXCALL &cVar)++
   MXCALL &cVar.()
   MXCALL &cVar.++
   (MXCALL &cVar.)++
   MXCALL &cVar.1 ()
   MXCALL &cVar.1 ++
   (MXCALL &cVar.1) ++
RETURN
