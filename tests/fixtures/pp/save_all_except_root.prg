#command SAVE ALL EXCEPT <p> TO <(f)> => __mvSave( <(f)>, <(p)>, .f. )
#command SAVE TO <(f)> ALL EXCEPT <p> => __mvSave( <(f)>, <(p)>, .f. )

PROCEDURE Main()
   SAVE ALL EXCEPT A TO A
RETURN
