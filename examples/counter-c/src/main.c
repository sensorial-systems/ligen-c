#include <Counter.h>
#include <CString.h>
#include <stdio.h>
#include <assert.h>

#define assert_eq(a, b) { printf("assert(%d == %d)\n", a, b); assert(a == b); }

#ifdef __cplusplus
extern "C" {
#endif
char* Counter_append(char* a, char* b);
#ifdef __cplusplus
}
#endif

int main(int argc, char **argv) {
    Counter counter = Counter_new(2); // it only works because the struct has the size of u32.
    assert_eq(Counter_get_count(counter), 2);
    Counter_count(counter, 1);
    assert_eq(Counter_get_count(counter), 3);
    Counter_count(counter, 3);
    assert_eq(Counter_get_count(counter), 6);
    printf("%s\n", Counter_append("Hello ", "world!"));
//    Counter_destroy(&counter);

    const char* s = "Hello!";
    printf("%p\n", s);
    CString string = CString_new(s); // it doesn't work because the CString type is far more complex. We want to create this object, forget about it
    printf("Result: %s\n", CString_as_ptr(string));
    return 0;
}