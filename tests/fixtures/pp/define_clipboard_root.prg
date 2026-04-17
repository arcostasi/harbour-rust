#command DEFINE CLIPBOARD <oClp>
   [ FORMAT <format:TEXT,OEMTEXT,BITMAP,DIF> ]
   [ OF <oWnd> ]
   =>
   <oClp> := TClipboard():New([UPPER(<(format)>)] [,<oWnd>] )

PROCEDURE Main()
   DEFINE CLIPBOARD oC OF oD FORMAT TEXT
RETURN
