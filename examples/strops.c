#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int booltmp = 0;
int main() {
int r2 = 5;
int *r0;
int r3 = 2;
int r4 = 10;
int r5 = 1;
int r6 = 1;
int r7;
r0 = (int *) malloc(r2);
r0[r3] = r4;
r7 = r5 + r6;
printf("%d\n", r0[r7]);
free(r0);
return 0;
}