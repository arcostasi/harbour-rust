#command @ <row>, <col> GET <var> RANGE <low>, <high> PICTURE <pic> => SetPos( <row>, <col> ) ; AAdd( GetList, _GET_( <var>, <"var">, <pic>, {| _1 | RangeCheck( _1,, <low>, <high> ) }, ) ) ; ATail(GetList):Display()
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
   @ 2,2 GET a RANGE 0,100 PICTURE "X"
RETURN
