#ifndef _TABLE_H

#define MAX_TABLE_SIZE 1024

typedef struct Table {
  char * names[MAX_TABLE_SIZE];
  int value[MAX_TABLE_SIZE];
  int count;
} Table;

void table_init(Table *t);
void table_store(Table *t, char key[], int val);
int table_get(Table *t, char n[]);

#define _TABLE_H
#endif
