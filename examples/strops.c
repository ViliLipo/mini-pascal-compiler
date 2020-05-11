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
char *r0;
r0 = (char *) malloc(256);
char *r1;
r1 = malloc(9);
strcpy(r1, "Kenguru");
char *r2;
r2 = malloc(10);
strcpy(r2, " Loikkaa");
char *r3;
char *r4;
r4 = (char *) malloc(256);
char *r5;
r5 = malloc(13);
strcpy(r5, " korkealle!");
char *r6;
int r8;
char *r9;
r9 = malloc(3);
strcpy(r9, "A");
int r10;
r3 = (char *) malloc(256);
strcpy(r3,r1);
strcat(r3,r2);
strcpy(r0, r3);
r6 = (char *) malloc(256);
strcpy(r6,r0);
strcat(r6,r5);
strcpy(r4, r6);
printf("%s\n", r4);
booltmp = strcmp(r9, r0);
r10 = booltmp < 0;
r8 = r10;
printf("%d\n", r8);
return 0;
}