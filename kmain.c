void kmain() {
    int i;
    int limit = 80 * 25 * 2;
    char *fb = (char *) 0x000B8000;
    char coffee[8] = { 'c', 'o', 'f', 'f', 'e', 'e', ' ', 0};

    for (i = 0; i < limit; i++) {
        fb[i * 2] = coffee[i % 7];
        fb[i * 2 + 1] = 6 << 4 | 1;
    }
}
