#command @ <row>, <col> GET <var> PICTURE <pic> RANGE <low>, <high> WHEN <when> => SetPos( <row>, <col> ) ; AAdd( GetList, _GET_( <var>, <"var">, <pic>, {| _1 | RangeCheck( _1,, <low>, <high> ) }, <{when}> ) )     ; ATail(GetList):Display()
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
   @ 2,4 GET a PICTURE "X" RANGE 0,100 WHEN .T.
RETURN
