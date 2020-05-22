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

int main() {
int r1;
int *r2;
char *r3;
int r4;
int r5;
char *r6;
int r7;
int r8;
r1 = 10;
int r2_size = r1;
r2 = (int*) malloc(r1 * sizeof(int));
r3 = (char*) malloc(13 * sizeof(char));
strcpy(r3, "Anna numero");
int r3_size = 13;
printf("%s ", r3);printf("\n");
r4 = 0;
r5 = 1;
scanf(" %d %d",  &r2[r4], &r2[r5]);
r6 = (char*) malloc(13 * sizeof(char));
strcpy(r6, "Numero on: ");
int r6_size = 13;
printf("%s ", r6);r7 = 0;
printf("%d ", r2[r7]);r8 = 1;
printf("%d ", r2[r8]);printf("\n");
return 0;}
