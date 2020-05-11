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
int r1 = 1;
int r2 = 1;
int r3;
int r4 = 1;
int r5 = 1;
int r6;
r3 = r1 == r2;
r0 = r3;
mp_assert(r0, "On line 4\n");
r6 = r4 < r5;
r0 = r6;
mp_assert(r0, "On line 6\n");
return 0;
}