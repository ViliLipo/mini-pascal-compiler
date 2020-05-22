#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>


void mp_assert(int condition, char* message) {
  if (! condition) {
    printf("Assert failed:\n");
    printf("\t%s", message);
    exit(1);
  }
}

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
int r1;
int r2;
int *r3;
int r4;
int *r5;
int r6;
int r7;
int r8;
int r9;
int r10;
int r11;
int r12;
int r13;
int r14;
int r15;
int r16;
int r17;
r2 = 5;
int r3_size = r2;
r3 = (int*) malloc(r2 * sizeof(int));
r4 = 5;
int r5_size = r4;
r5 = (int*) malloc(r4 * sizeof(int));
r6 = 0;
r1 = r6;
label0:
r7 = 5;
r8 = r1 < r7;
if (r8 != 1 ) { goto label1; }
r9 = 5;
r10 = r9 - r1;
r3[r1] = r10;
r11 = 1;
r12 = r1 + r11;
r1 = r12;
goto label0;label1:
r13 = 0;
r1 = r13;
memcpy(r5, r3, r5_size * sizeof(int));
label2:
r14 = 5;
r15 = r1 < r14;
if (r15 != 1 ) { goto label3; }
printf("%d ", r5[r1]);printf("\n");
r16 = 1;
r17 = r1 + r16;
r1 = r17;
goto label2;label3:
return 0;}
