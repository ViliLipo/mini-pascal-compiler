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
int r1 = 0;
int r2 = 10;
int r3;
int r4 = 1;
int r5;
int r6 = 5;
int r7;
int r8;
int r9 = 5;
r0 = r1;
label0:
r3 = r0 < r2;
if (r3 != 1 ) { goto label1; }
r5 = r0 + r4;
r0 = r5;
r7 = r0 < r6;
if (r7 != 1 ) { goto label2; }
r8 = r9;
printf("%d\n", r8);
goto label3;
label2:
printf("%d\n", r0);
label3:
goto label0;label1:
return 0;
}