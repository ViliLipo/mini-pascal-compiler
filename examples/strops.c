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
char *r0;
r0 = (char *) malloc(256);
char *r1;
r1 = malloc(9);
strcpy(r1, "Kenguru");
char *r2;
r2 = malloc(10);
strcpy(r2, " Loikkaa");
char *r3;
int r4;
char *r5;
r5 = malloc(3);
strcpy(r5, "A");
int r6;
r3 = (char *) malloc(256);
strcpy(r3,r1);
strcat(r3,r2);
strcpy(r0, r3);
printf("%s\n", r0);
booltmp = strcmp(r5, r0);
r6 = booltmp < 0;
r4 = r6;
printf("%d\n", r4);
return 0;
}