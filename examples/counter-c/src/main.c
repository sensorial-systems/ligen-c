#include <Counter.h>
#include <stdio.h>
#include <assert.h>

#define assert_eq(a, b) { printf("assert(%d == %d)\n", a, b); assert(a == b); }

int main(int argc, char **argv) {
    Counter counter = Counter_new(2);
    assert_eq(Counter_get_count(&counter), 2);
    Counter_count(&counter, 1);
    assert_eq(Counter_get_count(&counter), 3);
    Counter_count(&counter, 3);
    assert_eq(Counter_get_count(&counter), 6);
    return 0;
}