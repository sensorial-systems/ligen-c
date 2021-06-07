#include <Counter.h>
#include <RString.h>
#include <stdio.h>
#include <assert.h>

#define assert_eq(a, b) { printf("assert(%d == %d)\n", a, b); assert(a == b); }

int main(int argc, char **argv) {
    Counter counter = Counter_new(2);
    assert_eq(Counter_get_count(counter), 2);
    Counter_count(counter, 1);
    assert_eq(Counter_get_count(counter), 3);
    Counter_count(counter, 3);
    assert_eq(Counter_get_count(counter), 6);
//    printf("%s\n", Counter_append("Hello ", "world!"));
//    Counter_destroy(&counter);

    const char* s = "Hello!";
    printf("%p\n", s);
    RString string = RString_new(s);
    printf("Result: %s\n", RString_as_ptr(string));
//    CString_destroy(string);

    Person person = Person_new("First", "Last");

    RString fullName = Person_full_name(person);
    printf("Full name: %s", RString_as_ptr(fullName));
//    RString_destroy(fullName);

//    Person_destroy(person);

    return 0;
}
