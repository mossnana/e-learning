#include "linkedlist.h"
#include <stdio.h>
#include <stdlib.h>

Node *new_node(Node *prev, int value, Node *next) {
  Node *node = malloc(sizeof(Node));
  node->prev = prev;
  node->data = value;
  node->next = next;
  return node;
}

Node *add_node_before(Node *curr, int value) {
  if (curr == NULL)
    return NULL;
  Node *node = new_node(NULL, value, curr);
  curr->prev = node;
  return node;
}

void print_node(Node *curr) {
  if (curr == NULL)
    return;

  printf("[%d]", curr->data);
  curr = curr->next;

  while (curr != NULL) {
    printf(" -> [%d]", curr->data);
    curr = curr->next;
  }

  printf("\n");
}

void remove_node(Node *curr) {
  Node *deleteNode = curr;
  Node *nextNode = deleteNode->next;
  Node *prevNode = deleteNode->prev;
  if (prevNode != NULL) {
    prevNode->next = nextNode;
  }
  if (nextNode != NULL) {
    nextNode->prev = prevNode;
  }
  free(deleteNode);
}

void remove_node_by_value(Node *curr, int value) {
  while (curr != NULL) {
    if (curr->data == value)
      return remove_node(curr);
    curr = curr->next;
  }
}

void remove_node_by_index(Node *curr, int index) {
  if (index < 0)
    return;
  int i = 0;
  while (curr != NULL) {
    if (i == index) {
      return remove_node(curr);
    }
    curr = curr->next;
    i++;
  }
}
