int simple_while(int a) {
    int b = 0;
    while (a > 0) {
        b = b + 1;
        a = a - 1;
    }
    return b;
}
