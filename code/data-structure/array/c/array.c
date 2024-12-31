// ベクトルの実装

#include <stdbool.h>
#include <stdio.h>
#include <strings.h>

// ベクトル型
struct Vector {
  // 配列へのポインタ
  int *pointer_to_array;
  // 配列のアイテムの数
  int size;
  // 配列の長さ(保有上限)
  int capacity;
};

// ベクトルのアイテム数を取得
int size(struct Vector *vector) { return vector->size; }

// ベクトルの保持できるアイテム数を取得
int capacity(struct Vector *vector) { return vector->capacity; }

// ベクトルが空か否かの判定
bool is_empty(struct Vector *vector) {
  if (vector->size == 0) {
    return true;
  } else {
    return false;
  }
}

// 指定されたインデックスの値を取得する
// int at(struct Vector *vector, int index) {
//   // indexが範囲内かの判定
//   if (index < 0 || vector->size - 1 < index) {
//     // indexが範囲外である
//     // 爆発する。
//     return -999;
//   }
//   return vector->pointer_to_array;
// }

// サイズを見て、サイズ+1に要素を挿入
// サイズが不足した場合、リサイズ(サイズの拡大)が必要。
// 追加した後に、サイズを加算する
void push(struct Vector *vector, int item) {
  int array_size = vector->size;
  *(vector->pointer_to_array + array_size) = item;
  vector->size = array_size + 1;
}

// 指定したインデックスの値を取得する。
// 範囲外アクセスの場合は爆発する。
int at(struct Vector *vector, int index) {
  // インデックスのチェック
  int vector_size = vector->size;
  if (vector_size <= index) {
    return -999;
  }
  // 要素の取得
  return *(vector->pointer_to_array + index);
}

int main() {
  // vectorで参照する先の配列(初期値)
  int actual_array[16];

  // vectorを宣言、初期化
  struct Vector vector = {(int *)&actual_array, 0, 16};
  printf("initialize vector!!\n");

  // vectorのサイズは?
  int vec_size = size(&vector);
  printf("vector size is: %d\n", vec_size);

  // vectorのキャパシティは?
  int vec_cap = capacity(&vector);
  printf("vector capacity is: %d\n", vec_cap);

  // 初期化後は空
  bool is_vec_empty = is_empty(&vector);
  if (!is_vec_empty) {
    printf("error. vector is not empty.");
    return -1;
  }

  // アイテムのプッシュ
  push(&vector, 8);
  // 追加した後はからでない。
  bool is_vec_empty_2 = is_empty(&vector);
  if (is_vec_empty_2) {
    printf("error. vector is empty.");
    return -1;
  }

  // プッシュしたアイテムを取得
  int item_0 = at(&vector, 0);
  printf("item_0: %d\n", item_0);

  printf("finish.");

  return 0;
}
