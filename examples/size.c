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
int *r2;
int r3;
int r2_size = r1;
r2 = (int*) malloc(r1 * sizeof(int*));
r3 = r2_size;
printf("%d\n", r3);
return 0;
}