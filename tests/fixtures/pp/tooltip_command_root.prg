#define RED {255,0,0}
#xcommand SET TOOLTIP TO <color> OF <form> => SM( TTH (<"form">), 1, RGB(<color>\[1], <color>\[2\], <color>[, <color>\[ 3 \] ]), 0)

PROCEDURE Main()
   SET TOOLTIP TO RED OF form1
RETURN
