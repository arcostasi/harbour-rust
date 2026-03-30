FUNCTION Bump()
   STATIC count := 0

   count := count + 1

   RETURN count

PROCEDURE Main()
   ? Bump()
   ? Bump()
   ? Bump()
RETURN
