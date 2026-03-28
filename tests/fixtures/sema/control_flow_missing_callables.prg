PROCEDURE Main()
   LOCAL counter := 0

   IF counter == 0
      MissingBranch()
   ENDIF

   DO WHILE counter < 3
      MissingLoop(counter)
      counter++
   ENDDO

   FOR counter := 1 TO 2
      MissingStep()
   NEXT

   RETURN MissingTail()
