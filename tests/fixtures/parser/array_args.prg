PROCEDURE Main()

   LOCAL items := { 10, 20, 30 }

   ? Len( items )
   ? items[ 2 ]
   items[ 2 ] := 99
   ? items[ 2 ]
   ShowArray( items )

   RETURN

PROCEDURE ShowArray( arr )

   LOCAL i

   FOR i := 1 TO Len( arr )
      ? arr[ i ]
   NEXT

   RETURN
