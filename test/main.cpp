#include "add.h"
#include <iostream>

int main() {
    std::cout << "Enter a number: ";

    int x{};
    std::cin >> x;

    std::cout << "You entered " << x;
    std::cout << add(5, 5);

    return 0;
}
