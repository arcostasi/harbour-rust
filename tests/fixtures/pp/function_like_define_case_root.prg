#define F1( n ) F2( n, N )
#define F3( nN, Nn ) F2( nN, Nn, NN, nn, N, n )

PROCEDURE Main()
   ? F1( 1 )
   ? F3( 1, 2 )
RETURN
