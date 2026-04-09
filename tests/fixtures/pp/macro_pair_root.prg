#command FOO <x:&> FOO <y:&> => <(x)>+<(y)>
#translate BAR <x:&> BAR <y:&> => <(x)>+<(y)>
PROCEDURE Main()
   FOO &cVar FOO &var.
   BAR &cVar BAR &var.
   FOO &cVar FOO &var.+1
   BAR &cVar BAR &var.+1
RETURN
