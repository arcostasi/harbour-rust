#command MCOMMAND <x> => normal_c( <"x"> )
#command MCOMMAND <x:&> => macro_c( <(x)> )
PROCEDURE Main()
   MCOMMAND &cVar.+1
   MCOMMAND &cVar. .AND.  .T.
   MCOMMAND &cVar.++
   MCOMMAND &cVar.-=2
   MCOMMAND &cVar .AND.  .T.
   MCOMMAND & (cVar) +1
RETURN
