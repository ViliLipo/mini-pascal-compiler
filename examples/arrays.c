#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int booltmp = 0;
int main() {
int r0;
int r1 = 5;
int *r2;
int r3 = 5;
int *r4;
int r5 = 0;
int r6 = 5;
int r7;
int r8 = 5;
int r9;
int r10 = 1;
int r11;
int r12 = 0;
int r13 = 5;
int r14;
int r15 = 1;
int r16;
r2 = (int *) malloc(r1);
r4 = (int *) malloc(r3);
r0 = r5;
label0:
r7 = r0 < r6;
if (r7 != 1 ) { goto label1; }
r9 = r8 - r0;
r2[r0] = r9;
r11 = r0 + r10;
r0 = r11;
goto label0;label1:
r0 = r12;
memcpy(r4, r2, r3 * sizeof(int));
label2:
r14 = r0 < r13;
if (r14 != 1 ) { goto label3; }
printf("%d\n", r4[r0]);
r16 = r0 + r15;
r0 = r16;
goto label2;label3:
free(r2);
free(r4);
return 0;
}