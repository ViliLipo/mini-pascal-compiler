#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>


void mp_assert(int condition, char* message) {
  if (! condition) {
    printf("Assert failed:\n");
    printf("\t%s", message);
    exit(0);
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
int r0;
int r1 = 10;
int r2 = 10;
int r3;
int r4 = 11;
int r5;
int r6 = 9;
int r7;
int r8 = 10;
int r9;
int r10 = 11;
int r11;
int r12 = 10;
int r13;
int r14 = 9;
int r15;
int r16 = 9;
int r17;
int r18 = 5;
int r19 = 5;
int r20;
int r21 = 10;
int r22;
int r23 = 6;
int r24 = 2;
int r25;
int r26 = 3;
int r27;
int r28 = 6;
int r29 = 2;
int r30;
int r31 = 0;
int r32;
int r33 = 6;
int r34 = 4;
int r35;
int r36 = 2;
int r37;
r0 = r1;
r3 = r2 == r0;
mp_assert(r3, "On line 4\n");
r5 = r0 < r4;
mp_assert(r5, "On line 5\n");
r7 = r0 > r6;
mp_assert(r7, "On line 6\n");
r9 = r0 <= r8;
mp_assert(r9, "On line 7\n");
r11 = r0 <= r10;
mp_assert(r11, "On line 8\n");
r13 = r0 >= r12;
mp_assert(r13, "On line 9\n");
r15 = r0 >= r14;
mp_assert(r15, "On line 10\n");
r17 = r0 != r16;
mp_assert(r17, "On line 11\n");
r20 = r18 + r19;
r22 = r20 == r21;
mp_assert(r22, "On line 12\n");
r25 = r23 / r24;
r27 = r25 == r26;
mp_assert(r27, "On line 13\n");
r30 = r28 % r29;
r32 = r30 == r31;
mp_assert(r32, "On line 14\n");
r35 = r33 % r34;
r37 = r35 == r36;
mp_assert(r37, "On line 15\n");
return 0;
}