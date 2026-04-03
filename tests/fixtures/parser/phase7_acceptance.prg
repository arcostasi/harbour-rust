STATIC s_total := 0

PROCEDURE Main()
   LOCAL names := {"Alice", "Bob", "Charlie"}
   LOCAL i

   FOR i := 1 TO Len(names)
      IF Len(names[i]) > 3
         s_total += 1
         ? Upper(names[i]) + " tem nome longo"
      ELSE
         ? names[i] + " tem nome curto"
      ENDIF
   NEXT

   ? "Total de nomes longos: " + LTrim(Str(s_total))
   ? "Tipo do array: " + ValType(names)
RETURN
