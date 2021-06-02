#include <Counter.h>
#include <stdio.h>
#include <assert.h>

int main(int argc, char **argv) {
    Counter counter = Counter_new();
    assert(Counter_get_count(&counter) == 0);
    Counter_count(&counter, 1);
    assert(Counter_get_count(&counter) == 1);
    Counter_count(&counter, 3);
    assert(Counter_get_count(&counter) == 4);
    return 0;
}