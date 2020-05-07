#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int booltmp = 0;
int main() {
int r0;
int r3 = 5;
int *r1;
int r4 = 0;
int r5 = 5;
int r6;
int r7 = 5;
int r8;
int r9 = 1;
int r10;
int r11 = 0;
int r12 = 5;
int r13;
int r14 = 1;
int r15;
r1 = (int *) malloc(r3);
r0 = r4;
label0:
r6 = r0 < r5;
if (r6 != 1 ) { goto label1; }
r8 = r7 - r0;
r1[r0] = r8;
r10 = r0 + r9;
r0 = r10;
goto label0;label1:
r0 = r11;
label2:
r13 = r0 < r12;
if (r13 != 1 ) { goto label3; }
printf("%d\n", r1[r0]);
r15 = r0 + r14;
r0 = r15;
goto label2;label3:
free(r1);
return 0;
}