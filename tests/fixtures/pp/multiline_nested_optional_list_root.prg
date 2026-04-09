#xcommand SET <var1> [, <varN>] WITH <val> =>
<var1>:=<val> [; <varN>:=<val>]
#command AVG <x1> [, <xn>] TO <v1> [, <vn>]  =>
   AVERAGE( {||<v1>:=<v1>+<x1>} [, {||<vn>:=<vn>+<xn>} ] )
PROCEDURE Main()
   SET v1 WITH 0
   SET v1, v2, v3, v4 WITH 0
   AVG f1 TO s1
   AVG f1, f2 TO s1, s2
   AVG f1, f2, f3 TO s1, s2, s3
RETURN
