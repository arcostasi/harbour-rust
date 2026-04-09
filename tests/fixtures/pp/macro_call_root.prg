#xtranslate MXCALL <x:&> => (<x>)
#xtranslate MYCALL <x:&> <y> => <x>( <y>, 'mycall' )
#xtranslate MZCALL <x> <y> => <x>( <y>, "mzcall" )
PROCEDURE Main()
   MXCALL &cVar
   MXCALL &cVar++
   MYCALL &cVar &cVar
   MYCALL &cVar+1 &cVar
   MZCALL &cVar ++cVar
   MZCALL &cVar+1 &cVar
RETURN
