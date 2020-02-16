#include "table.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

void table_init(Table *t) {
    t->count = 0;
}
void table_store(Table *t, char key[], int val) {
    for (int i = 0; i < t->count; i ++) {
        if(strcmp(t->names[i], key) == 0) {
            t->value[i] = val;
            return;
        }
    }
    if(t->count == MAX_TABLE_SIZE) {
        printf("exceed max limit");
        exit(1);
    }
    t->names[t->count] = malloc(strlen(key));
    strcpy(t->names[t->count], key);
    t->value[t->count] = val;
    t->count ++;
}

int table_get(Table *t, char key[]) {
    for (int i = 0; i < t->count; i ++) {
        if(strcmp(t->names[i], key) == 0) {
            return t->value[i];
        }
    }

    return 0;
}
