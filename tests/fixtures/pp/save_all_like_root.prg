#command SAVE TO <(f)> ALL LIKE <p> => __mvSave( <(f)>, <(p)>, .t. )
#command SAVE ALL LIKE <p> TO <(f)> => __mvSave( <(f)>, <(p)>, .t. )

PROCEDURE Main()
   SAVE ALL LIKE A TO A
RETURN
