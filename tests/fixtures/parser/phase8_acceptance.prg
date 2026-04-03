PROCEDURE Main()
   PUBLIC g_total := 10
   PRIVATE counter := 4
   LOCAL cName := "g_total"

   ? Eval( {|x, y| x + y }, 2, 3 )
   ? Eval( {|| counter + 1 } )
   ? &cName
   ? &( cName )

   Bump()

   ? counter
   ? g_total
RETURN

PROCEDURE Bump()
   counter := counter + 1
   g_total := g_total + 5
RETURN
