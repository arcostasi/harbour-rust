#command _REGULAR_(<z>) => rm( <z> )
PROCEDURE Main()
   _REGULAR_(a)
   _REGULAR_("a")
   _REGULAR_(&a.1)
   _REGULAR_(&a)
   _REGULAR_(a[1])
RETURN
