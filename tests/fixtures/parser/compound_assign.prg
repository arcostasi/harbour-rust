// Compound assignment baseline

PROCEDURE Main()

   LOCAL total := 1
   STATIC factor := 2
   total += 3
   factor *= total

   RETURN factor
