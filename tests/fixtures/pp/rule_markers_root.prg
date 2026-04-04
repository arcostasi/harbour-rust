#command MAYBE [VALUE <value>] => Emit([<value>])
#command LISTIT <values,...> => ? #<values>
#command SET MODE <mode:ON,OFF> => ? #<mode>
PROCEDURE Main()
   MAYBE
   MAYBE VALUE n
   LISTIT a, b, c
   SET MODE ON
   SET MODE MAYBE
RETURN
