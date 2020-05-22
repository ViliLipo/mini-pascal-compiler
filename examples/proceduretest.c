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
char *r5;
r5 = (char*) malloc(20 * sizeof(char));
strcpy(r5, "Procedure jyrää!");
int r5_size = 20;
printf("%s ", r5);printf("\n");
printf("%d ", r4);printf("\n");
return;}
void r6(int r7) {
int r8;
int r9;
r8 = 10;
r9 = r7 + r8;
printf("%d ", r9);printf("\n");
}
void r10(char * r11) {
printf("%s ", r11);printf("\n");
}
int main() {
int r0 = 0;
int r1 = 1;
char *r12;
int r13;
int r14;
char *r15;
r12 = (char*) malloc(21 * sizeof(char));
strcpy(r12, "Main jyrähtää ja");
int r12_size = 21;
printf("%s ", r12);printf("\n");
r13 = 5;
r3( r13);
r14 = 3;
r6( r14);
r15 = (char*) malloc(18 * sizeof(char));
strcpy(r15, "YOLO toimi vittu");
int r15_size = 18;
r10( r15);
return 0;}
