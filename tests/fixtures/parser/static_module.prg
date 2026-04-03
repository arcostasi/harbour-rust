STATIC s_count := 0
STATIC s_cache

FUNCTION Increment()
   s_count := s_count + 1
   RETURN s_count

FUNCTION CacheType()
   RETURN ValType(s_cache)

PROCEDURE Main()
   ? Increment()
   ? Increment()
   ? s_count
   ? CacheType()
RETURN
