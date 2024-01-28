#include <stdio.h>

void fnD() 
{
    printf("This is function D\n");
}

void fnB() 
{
    printf("This is function B\n");
}

void fnC() 
{
    printf("This is function C\n");
    fnD();
}

void fnA() 
{
    printf("This is function A\n");
    fnB();
    fnC();
}


int main() 
{
    printf("Hello, World!\n");
    fnA();
    return 0;
}
