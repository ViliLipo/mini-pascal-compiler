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
char *r2;
char *r3;
char *r4;
char *r6;
char *r7;
char *r9;
int r10;
r2 = (char*) malloc(9 * sizeof(char));
strcpy(r2, "Kenguru");
int r2_size = 9;
r3 = (char*) malloc(10 * sizeof(char));
strcpy(r3, " Loikkaa");
int r3_size = 10;
r4 = (char *) malloc(256);
strcpy(r4,r2);
strcat(r4,r3);
r1 = r4;
r6 = (char*) malloc(13 * sizeof(char));
strcpy(r6, " korkealle!");
int r6_size = 13;
r7 = (char *) malloc(256);
strcpy(r7,r1);
strcat(r7,r6);
r5 = r7;
printf("%s\n", r5);
r9 = (char*) malloc(3 * sizeof(char));
strcpy(r9, "A");
int r9_size = 3;
r10 = r9 < r1;
r8 = r10;
printf("%d\n", r8);
return 0;
