#include <algorithm>
#include <cstdio>
#include <iostream>
using namespace std;
int main() {
    int l, r, x;
    scanf("%d %d %d", &l, &r, &x);
    printf("%d %d\n", l, r);
    fflush(stdout);
    int steps = 0;
    while (1) {
        char c;
        int y;
        scanf(" %c %d", &c, &y);
        steps ++;
        if (y < l || y > r) {
            printf("Error\n");
            fflush(stdout);
            continue;
        }
        if (c == '?') {
            if (y < x) {
                printf("L\n");
            } 
            if (y > x) {
                printf("R\n");
            }
            if (y == x) {
                printf("E\n");
            }
            fflush(stdout);
            continue;
        }
        if (c == '!') {
            printf("END\n");
            fflush(stdout);
            if (y == x) {
                printf("AC with %d steps\n", steps);
            } else {
                printf("WA with %d steps\n", steps);
            }
            fflush(stdout);
            break;
        }
        printf("Error\n");
        fflush(stdout);
    }
    return 0;
}