#include <stdio.h>
#include <string.h>
int main() {
char r0[256];
char r1[256] = "YOLO";
char r2[256] = " SWAG";
char r3[256];
strcpy(r0, r1);
strcpy(r3,r0);
strcat(r3,r2);
strcpy(r0, r3);
printf("%s\n", r0);
return 0;
}