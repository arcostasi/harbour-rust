PROCEDURE Main()
   LOCAL counter := 0

   IF missing_flag
      ? counter
   ENDIF

   DO WHILE counter < missing_limit
      ? counter
      counter++
   ENDDO

   FOR counter := 1 TO final_value
      ? inner_value
   NEXT

   RETURN missing_result
