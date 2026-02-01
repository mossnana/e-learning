#ifndef LINKED_LIST
#define LINKED_LIST

typedef struct Node {
  struct Node *prev;
  int data;
  struct Node *next;
} Node;

Node *new_node(Node *prev, int value, Node *next);
Node *add_node_before(Node *curr, int value);
void print_node(Node *curr);
void remove_node_by_value(Node *curr, int value);
void remove_node_by_index(Node *curr, int index);

#endif
