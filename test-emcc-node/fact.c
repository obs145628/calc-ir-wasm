

int fact(int x)
{
  return x <= 1 ? 1 : x * fact(x-1);
}
