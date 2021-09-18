long local_exchange(long *xp, long y)
{
	long x  = *xp;
	*xp = y;
	return x;
}
