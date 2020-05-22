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
int r1;
int r2;
int r3;
int r4;
int r5;
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
int r20;
int r21;
int r22;
int r23;
int r24;
int r25;
int r26;
int r27;
int r28;
int r29;
int r30;
int r31;
int r32;
int r33;
int r34;
int r35;
int r36;
int r37;
int r38;
r2 = 10;
r1 = r2;
r3 = 10;
r4 = r3 == r1;
mp_assert(r4, "On line 4\n");
r5 = 11;
r6 = r1 < r5;
mp_assert(r6, "On line 5\n");
r7 = 9;
r8 = r1 > r7;
mp_assert(r8, "On line 6\n");
r9 = 10;
r10 = r1 <= r9;
mp_assert(r10, "On line 7\n");
r11 = 11;
r12 = r1 <= r11;
mp_assert(r12, "On line 8\n");
r13 = 10;
r14 = r1 >= r13;
mp_assert(r14, "On line 9\n");
r15 = 9;
r16 = r1 >= r15;
mp_assert(r16, "On line 10\n");
r17 = 9;
r18 = r1 != r17;
mp_assert(r18, "On line 11\n");
r19 = 5;
r20 = 5;
r21 = r19 + r20;
r22 = 10;
r23 = r21 == r22;
mp_assert(r23, "On line 12\n");
r24 = 6;
r25 = 2;
r26 = r24 / r25;
r27 = 3;
r28 = r26 == r27;
mp_assert(r28, "On line 13\n");
r29 = 6;
r30 = 2;
r31 = r29 % r30;
r32 = 0;
r33 = r31 == r32;
mp_assert(r33, "On line 14\n");
r34 = 6;
r35 = 4;
r36 = r34 % r35;
r37 = 2;
r38 = r36 == r37;
mp_assert(r38, "On line 15\n");
return 0;}
