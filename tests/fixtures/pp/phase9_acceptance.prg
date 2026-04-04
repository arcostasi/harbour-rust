#command EMIT <value> => ? <value>
#command EMITLIST <values,...> => ? #<values>
#command SET MODE <mode:ON,OFF> => ? #<mode>
#translate DOUBLE(<value>) => <value> + <value>
#translate SHOUT(<value>) => Upper(<value>)
PROCEDURE Main()
   LOCAL n := DOUBLE(3)
   EMIT n
   EMIT SHOUT("abc")
   EMITLIST 1, 2, 3
   SET MODE ON
RETURN
