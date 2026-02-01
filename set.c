#include "set.h"
#include "linkedlist.h"
#include <stdio.h>
#include <stdlib.h>

int hash(int key) { return abs(key) % TABLE_SIZE; }

void add_set(HashSet *set, int value) {
  int index = hash(value);

  Node *curr = set->buckets[index];
  while (curr != NULL) {
    if (curr->data == value)
      return;
    set->buckets[index] = add_node_before(curr, value);
  }

  set->buckets[index] = new_node(NULL, value, set->buckets[index]);
}
