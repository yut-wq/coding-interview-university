// ベクトルの実装

#include <stdbool.h>
#include <strings.h>

int main() { return 0; }

// ベクトル型
struct Vector {
  // 配列へのポインタ
  int *pointer_to_array;
  // 配列のアイテムの数
  int size;
  // 配列の長さ(保有上限)
  int capacity;
};

int array[16];

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
int at(struct Vector *vector, int index) {
  // indexが範囲内かの判定
  if (index < 0 || vector->size - 1 < index) {
    // indexが範囲外である
    // 爆発する。
    return -999;
  }
  return vector->pointer_to_array;
}
