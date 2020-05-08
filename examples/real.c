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
double r0;
double r1 = 3.14;
double r2;
double r3 = 2.0;
double r4;
r0 = r1;
r4 = r0 / r3;
r2 = r4;
printf("%f\n", r2);
return 0;
}