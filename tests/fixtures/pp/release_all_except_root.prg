#command RELEASE <v,...> => __mvXRelease( <"v"> )
#command RELEASE ALL => __mvRelease( "*", .t. )
#command RELEASE ALL LIKE <p> => __mvRelease( #<p>, .t. )
#command RELEASE ALL EXCEPT <p> => __mvRelease( #<p>, .f. )

PROCEDURE Main()
   RELEASE ALL EXCEPT A
RETURN
