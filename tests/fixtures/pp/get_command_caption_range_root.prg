#command @ <row>, <col> GET <var> [<exp,...>] RANGE <low>, <high> [<nextexp,...>] => @ <row>, <col> GET <var> [ <exp>] VALID {| _1 | RangeCheck( _1,, <low>, <high> ) } [ <nextexp>]
#command @ <row>, <col> GET <var>
                        [PICTURE <pic>]
                        [VALID <valid>]
                        [WHEN <when>]
                        [CAPTION <caption>]
                        [MESSAGE <message>]
                        [SEND <msg>]

      => SetPos( <row>, <col> )
       ; AAdd( GetList,
              _GET_( <var>, <"var">, <pic>, <{valid}>, <{when}> ) )
      [; ATail(GetList):Caption := <caption>]
      [; ATail(GetList):CapRow  := ATail(Getlist):row
       ; ATail(GetList):CapCol  := ATail(Getlist):col -
                              __CapLength(<caption>) - 1]
      [; ATail(GetList):message := <message>]
      [; ATail(GetList):<msg>]
       ; ATail(GetList):Display()

PROCEDURE Main()
   @ 1,5 GET a PICTURE "X" WHEN .T. CAPTION "myget" RANGE 0,100
RETURN
