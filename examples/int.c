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

void alloc_str_array(char** str_array, int size, int string_size) {
  int i = 0;
  while (i < size) {
    str_array[i] = (char *) malloc(string_size);
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
int r0 = 0;
int r1 = 1;
int r3;
int r4;
r4 = 10;
int r5;
r5 = 10;
short r6;
int r7;
r7 = 11;
short r8;
int r9;
r9 = 9;
short r10;
int r11;
r11 = 10;
short r12;
int r13;
r13 = 11;
short r14;
int r15;
r15 = 10;
short r16;
int r17;
r17 = 9;
short r18;
int r19;
r19 = 9;
short r20;
int r21;
r21 = 5;
int r22;
r22 = 5;
int r23;
int r24;
r24 = 10;
short r25;
int r26;
r26 = 6;
int r27;
r27 = 2;
int r28;
int r29;
r29 = 3;
short r30;
int r31;
r31 = 6;
int r32;
r32 = 2;
int r33;
int r34;
r34 = 0;
short r35;
int r36;
r36 = 6;
int r37;
r37 = 4;
int r38;
int r39;
r39 = 2;
short r40;
r3 = r4;
r6 = r5 == r3;
mp_assert(r6, "On line 5\n");
r8 = r3 < r7;
mp_assert(r8, "On line 6\n");
r10 = r3 > r9;
mp_assert(r10, "On line 7\n");
r12 = r3 <= r11;
mp_assert(r12, "On line 8\n");
r14 = r3 <= r13;
mp_assert(r14, "On line 9\n");
r16 = r3 >= r15;
mp_assert(r16, "On line 10\n");
r18 = r3 >= r17;
mp_assert(r18, "On line 11\n");
r20 = r3 != r19;
mp_assert(r20, "On line 12\n");
r23 = r21 + r22;
r25 = r23 == r24;
mp_assert(r25, "On line 13\n");
r28 = r26 / r27;
r30 = r28 == r29;
mp_assert(r30, "On line 14\n");
r33 = r31 % r32;
r35 = r33 == r34;
mp_assert(r35, "On line 15\n");
r38 = r36 % r37;
r40 = r38 == r39;
mp_assert(r40, "On line 16\n");
return 0;}
