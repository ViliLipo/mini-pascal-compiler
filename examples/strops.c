#include <stdio.h>
#include <string.h>
int booltmp = 0;
int main() {
char r0[256];
char r1[256] = "YOLO";
char r2[256] = " SWAG";
char r3[256];
char r4[256];
char r5[256];
char r6[256] = "AASI";
char r7[256] = "KOIRA";
int r8;
strcpy(r0, r1);
strcpy(r3,r0);
strcat(r3,r2);
strcpy(r0, r3);
strcpy(r4, r6);
strcpy(r5, r7);
printf("%s\n", r0);
printf("\n", r8);
return 0;
}