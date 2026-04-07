#command MCOMMAND <x> => normal_c( <"x"> )
#command MCOMMAND <x:&> => macro_c( <(x)> )
PROCEDURE Main()
   MCOMMAND &cVar
   MCOMMAND &cVar.
   MCOMMAND &(cVar)
   MCOMMAND & (cVar)
   MCOMMAND &cVar&cVar
   MCOMMAND &cVar+1
   MCOMMAND &(cVar) +1
RETURN