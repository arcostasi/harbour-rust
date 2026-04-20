#command RELEASE <v,...> => __mvXRelease( <"v"> )
#command RELEASE ALL => __mvRelease( "*", .t. )
#command RELEASE ALL LIKE <p> => __mvRelease( #<p>, .t. )

PROCEDURE Main()
   RELEASE ALL LIKE A
RETURN
