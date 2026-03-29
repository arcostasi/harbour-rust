PROCEDURE Main()
   LOCAL same := { 1 }
   LOCAL alias := same
   LOCAL other := { 1 }
   LOCAL left := {}
   LOCAL right := {}

   ? same == same
   ? same == alias
   ? same == other
   ? left == right
RETURN
