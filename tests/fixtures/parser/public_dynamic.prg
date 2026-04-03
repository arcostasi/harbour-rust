PROCEDURE Main()
   PUBLIC g_count := 10
   Helper()
   ? g_count
RETURN

PROCEDURE Helper()
   g_count := g_count + 1
RETURN
