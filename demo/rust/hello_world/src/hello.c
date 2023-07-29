#include <stdio.h>
#include <stdlib.h>

void hello(void)
{
    printf("[%s][%s] Hello World\r\n", __DATE__, __TIME__);
}