#include "linkedlist.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
  int max_loop = 10;

  Node *node = malloc(sizeof(Node));
  node->data = 1;
  node->prev = NULL;
  node->next = NULL;
  print_node(node);

  for (int i = 2; i < max_loop; i++) {
    node = add_node_before(node, i);
    print_node(node);
  }

  Node *tempPrint = node;
  while (tempPrint != NULL) {
    print_node(tempPrint);
    tempPrint = tempPrint->next;
  }

  while (node != NULL) {
    Node *temp = node;
    node = node->next;
    free(temp);
  }

  return 0;
}
