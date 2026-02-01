#include "linkedlist.h"
#ifndef HASH_SET
#define HASH_SET
#define TABLE_SIZE 100

typedef struct {
  Node *buckets[TABLE_SIZE];
} HashSet;

void add_set(HashSet *set, int value);

#endif
