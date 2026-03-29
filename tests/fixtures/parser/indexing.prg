// Array indexing baseline

FUNCTION Pick(row, col)

   LOCAL matrix := { { 10, 20 }, { 30, 40 } }

   RETURN matrix[row][1 + col]
