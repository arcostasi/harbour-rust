#command EMIT <value> => ? <value>
#translate DOUBLE(<value>) => <value> + <value>
PROCEDURE Main()
   LOCAL n := DOUBLE(3)
   EMIT n
   EMIT DOUBLE(4)
RETURN
