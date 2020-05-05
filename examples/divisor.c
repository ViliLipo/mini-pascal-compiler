#include <stdio.h>
#include <string.h>
int main() {
int r0;
int r1;
int r2 = 3;
int r3 = 5;
int r4 = 1000;
int r5 = 3;
int r6;
int r7;
int r8;
int r9 = 1000;
int r10;
int r11;
int r12 = 3;
int r13 = 8;
int r14;
int r15;
r0 = r2;
r6 = r4 / r5;
r7 = r3 * r6;
r8 = r0 + r7;
r1 = r8;
r10 = r1 < r9;
if (r10 != 1 ) { goto label0; }
printf("%d\n", r0);
goto label1;
label0:
goto label1;
label1:
r14 = r12 + r13;
r11 = r14;
r15 = r0 + r11;
r0 = r15;
printf("%d\n", r0);
return 0;
}