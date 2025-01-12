#include <stdio.h>
#include <stdlib.h>

void enqueue(int value) {}
int dequeue() {}
void empty() {}

struct Queue
{
    int head;
    int tail;
    int list[32];
};

int main()
{
    printf("initialize queue\n");
    int *list = malloc(16);
    if (list == NULL)
    {
        printf("something wrong.\n");
        return -1;
    }

    struct Queue queue =
        {
            NULL,
            NULL,
            list,
        };

    int value = 11;
    enqueue(value);
    int dequeued_value = dequeue();
    if (value != dequeued_value)
    {
        printf("value is wrong.\n");
        return -1;
    }

    printf("exit\n");
    return 0;
}
