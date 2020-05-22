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
int *r2;
int r3;
int r5;
int r4;
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
int r18;
int r19;
int r21;
int r22;
int r20;
int r23;
char *r24;
r1 = 5;
int r2_size = r1;
r2 = (int*) malloc(r1 * sizeof(int));
r5 = 3;
r4 = 2;
r2[r4] = r5;
r6 = 2;
r7 = 5;
r8 = r2[r6] + r7;
r9 = 1000;
r10 = 3;
r11 = r9 / r10;
r12 = r8 * r11;
r3 = r12;
r13 = 1000;
r14 = r3 < r13;
if (r14 != 1 ) { goto label0; }
r15 = 2;
printf("%d ", r2[r15]);printf("\n");
goto label1;
label0:
r17 = 3;
r18 = 8;
r19 = r17 + r18;
r16 = r19;
r21 = 2;
r22 = r2[r21] + r16;
r20 = 2;
r2[r20] = r22;
r23 = 2;
printf("%d ", r2[r23]);r24 = (char*) malloc(7 * sizeof(char));
strcpy(r24, "lmaoo");
int r24_size = 7;
printf("%s ", r24);printf("\n");
label1:
return 0;}
