#command SAVE TO <(f)> ALL LIKE <p> => __mvSave( <(f)>, <(p)>, .t. )
#command SAVE ALL LIKE <p> TO <(f)> => __mvSave( <(f)>, <(p)>, .t. )

PROCEDURE Main()
   SAVE TO A ALL LIKE A
RETURN
