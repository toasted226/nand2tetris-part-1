#include <stdio.h>

int main(void) {
    int x = 0;
    int y = 1;
    int z = 0;

    while (1) {
        z = x + y;
        x = y;
        y = z;
    }

    return 0;
}