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

void r3(int r4) {
int r5;
int r6;
int r7;
int r8;
r5 = 1;
r6 = r4 <= r5;
if (r6 != 1 ) { goto label0; }
printf("%d ", r4);printf("\n");
goto label1;
label0:
printf("%d ", r4);printf("\n");
r7 = 1;
r8 = r4 - r7;
r3( r8);
label1:
return;
}
int main() {
int r0 = 0;
int r1 = 1;
int r9;
r9 = 10;
r3( r9);
return 0;}
