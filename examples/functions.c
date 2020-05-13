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
int r2;
int r3 = 1;
int r4 = 2;
printf("%d\n", r2);
return 0;
}