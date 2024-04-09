#include "add.h"
#include "subtract.h"
#include <iostream>

int main() {
    std::cout << "Enter a number: ";

    int x{};
    std::cin >> x;

    std::cout << "You entered: " << x << '\n';
    std::cout << add(5, 5) << '\n';
    std::cout << subtract(5, 3) << '\n';

    std::cin.get();
    return 0;
}
