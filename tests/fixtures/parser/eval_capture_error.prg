FUNCTION Build()
   LOCAL n := 2
   RETURN {|x| x + n }

PROCEDURE Main()
   ? Eval( Build(), 3 )
RETURN
