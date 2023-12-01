#include <stdio.h>

char readFile()
{
    FILE *fptr;
    fptr = fopen("example.txt", "r");
    char content[100];
    fgets(content, 100, fptr);
    fclose(fptr);
    return content;
}

int main()
{
    printf("Puzzle");
    return 0;
}