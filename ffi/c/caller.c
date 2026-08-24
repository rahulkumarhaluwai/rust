#include <stdio.h>

extern int task1_correct_ffi(void);
extern int task2_wrong_ffi(void);
extern int task3_safe_panic_wrapper(void);

int main(void) {
    printf("TASK 1: %d\n", task1_correct_ffi());

    printf("TASK 2: %d\n", task2_wrong_ffi());

    int result = task3_safe_panic_wrapper();

    printf("TASK 3: %d\n", result);

    if (result == -1) {
        printf("TASK 3: panic was caught; C process survived.\n");
    }

    return 0;
}