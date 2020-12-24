
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <vector>

class Node;

std::vector<Node *> node_index(1000000);

class Node {
 public:
  Node(int x): x_(x), next_(this) { node_index[x] = this; }
  ~Node() {}

  Node *Next() { return next_; }

  // O(1)
  Node *Insert(int x) {
    Node *other = new Node(x);
    other->next_ = next_;
    next_ = other;
    return other;
  }

  // O(len(others))
  void Insert(Node *others) {
    Node *next = next_;
    next_ = others;
    Node *end = others;
    while (end->next_ != others) {
      end = end->next_;
    }
    end->next_ = next;
  }

  // O(1)
  Node *Take3() {
    Node *one = next_;
    Node *two = next_->next_;
    Node *three = next_->next_->next_;
    next_ = three->next_;
    three->next_ = one;
    return one;
  }

  // O(len(taken))
  int ComputeDestination(Node *taken, int max) {
    int target = x_ - 1;
    while (true) {
      if (target == 0) {
        target = max;
      }
      if (taken->x_ == target ||
          taken->next_->x_ == target ||
          taken->next_->next_->x_ == target) {
        target--;
        continue;
      }
      return target;
    }
  }

  // O(1)
  Node* Find(int x) {
    return node_index[x];
  }

  void Print() {
    Node *node = this;
    int i = 0;
    do {
      printf("%d ", node->x_);
      node = node->next_;
      i++;
      if (i > 80) {
        printf("...");
        break;
      }
    } while (node != this);
    printf("\n");
  }

 private:
  Node(const Node& other) = delete;
  Node& operator=(const Node& other) = delete;
  Node(Node&& other) = delete;
  Node& operator=(Node&& other) = delete;

 private:
  int x_;
  Node *next_;
};

Node *Turn(Node *ring) {
  //printf("cups: ");
  //ring->Print();
  auto picked_up = ring->Take3();
  //printf("pick up: ");
  //picked_up->Print();
  int destination = ring->ComputeDestination(picked_up, 1000000);
  //printf("destination: %d\n", destination);
  Node* insertion = ring->Find(destination);
  insertion->Insert(picked_up);
  return ring->Next();
}

void Free(Node *root) {
  Node *node = root;
  do {
    Node *next = node->Next();
    delete node;
    node = next;
  } while (node != root);
}

Node *CreateInput() {
  Node *ring = new Node(8);
  Node *node = ring;
  node = node->Insert(7);
  node = node->Insert(1);
  node = node->Insert(3);
  node = node->Insert(6);
  node = node->Insert(9);
  node = node->Insert(4);
  node = node->Insert(5);
  node = node->Insert(2);
  for (int i = 10; i <= 1000000; i++) {
    node = node->Insert(i);
  }
  return ring;
}

int main(int argc, char **argv) {
  Node *ring = CreateInput();
  time_t start = time(nullptr);
  for (int i = 0; i < 10000000; i++) {
    if (i < 100 || i % 1000 == 0) {
      double remaining = 0;
      if (i > 0) {
        time_t now = time(nullptr);
        double elapsed = difftime(now, start);
        double seconds_per_move = elapsed / i;
        int moves_remaining = 10000000 - i;
        remaining = seconds_per_move * moves_remaining;
      }
      printf("-- move %d; remaining: %f seconds --\n", i, remaining);
    }
    ring = Turn(ring);
  }
  ring->Print();

  Node *answer = ring->Find(1)->Next();
  answer->Print();

  Free(ring);
}