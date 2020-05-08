#include <stdio.h>
#include <string.h>
#include <stdlib.h>

void alloc_str_array(char** str_array, int size) {
  int i = 0;
  while (i < size) {
    str_array[i] = (char *) malloc(256);
    i = i + 1;
  }
}

void free_str_array(char** str_array, int size) {
  int i = 0;
  while (i < size) {
    free(str_array[i]);
    i = i + 1;
  }
}

int booltmp = 0;

int main() {
int r0;
int r1 = 5;
char * *r2;
int r3 = 5;
char * *r4;
int r5 = 0;
int r6 = 5;
int r7;
char *r8;
r8 = malloc(6);
strcpy(r8, "AASI");
char *r9;
r9 = malloc(8);
strcpy(r9, " LMAOO");
char *r10;
int r11 = 1;
int r12;
int r13 = 0;
int r14 = 5;
int r15;
int r16 = 1;
int r17;
r2 = (char **) malloc(r1 * sizeof(char **));
alloc_str_array(r2, r1);
r4 = (char **) malloc(r3 * sizeof(char **));
alloc_str_array(r4, r3);
r0 = r5;
label0:
r7 = r0 < r6;
if (r7 != 1 ) { goto label1; }
r10 = (char *) malloc(256);
strcpy(r10,r8);
strcat(r10,r9);
strcpy(r2[r0], r10);
r12 = r0 + r11;
r0 = r12;
goto label0;label1:
r0 = r13;
memcpy(r4, r2, r3 * sizeof(char *));
label2:
r15 = r0 < r14;
if (r15 != 1 ) { goto label3; }
printf("%s\n", r4[r0]);
r17 = r0 + r16;
r0 = r17;
goto label2;label3:
return 0;
}