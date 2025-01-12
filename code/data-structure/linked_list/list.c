#include <stdio.h>
#include <stdlib.h>

/// @brief リストの1要素を格納する
struct Cell
{
    int element;
    struct Cell *next;
};

// リストの要素数を取得する
int size(struct Cell *list_head)
{
    int list_size = 0;
    struct Cell *now_cell = list_head;
    while (1)
    {
        list_size += 1;
        if (now_cell->next == NULL)
        {
            break;
        }
        now_cell = now_cell->next;
    }

    return list_size;
}

int main()
{
    // 連結リストの作成
    struct Cell cell_1, cell_2, cell_3;
    struct Cell *head;

    cell_1.next = &cell_2;
    cell_2.next = &cell_3;
    cell_3.next = NULL;
    head = &cell_1;

    int list_size = size(head);
    printf("list_size: %d\n", list_size);

    return 0;
}