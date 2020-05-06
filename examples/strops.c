#include <stdio.h>
#include <string.h>
int booltmp = 0;
int main() {
int r0 = 1;
int r1 = 1;
int r2;
int r3 = 5;
int r4 = 4;
int r5;
int r6;
char r7[256] = "Jeboii";
char r8[256] = "Nahbro";
r2 = r0 == r1;
r5 = r3 > r4;
r6 = r2 & r5;
if (r6 != 1 ) { goto label0; }
printf("%s\n", r7);
goto label1;
label0:
printf("%s\n", r8);
label1:
return 0;
}